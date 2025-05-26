use crate::internal_commands::InternalCommand;
use crate::internal_commands::InternalCommand::{DashboardViewerConnected, OperatorConnected};
use async_channel::{Receiver, Sender, unbounded};
use async_std::io::ReadExt;
use async_std::net::{TcpListener, TcpStream};
use async_std::sync::Mutex;
use async_std::task::sleep;
use bevy::prelude::{Commands, Resource};
use bevy::tasks::IoTaskPool;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use futures::{AsyncWriteExt, FutureExt};
use shared::{ClientActionCommand, ClientMessage, HelloState, OperatorMode, ServerEvent};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

const TARGET_SERVER_TICK: u64 = 32;
const SLEEP_TIME_FOR_TARGET_TICK: u64 = 1000 / TARGET_SERVER_TICK;

#[derive(Resource)]
pub struct ClientActionCommandReceiver {
    pub(crate) rx_client_action_commands: Receiver<(Uuid, ClientActionCommand)>,
}

#[derive(Resource)]
pub struct ServerEventSender {
    pub(crate) tx_server_events: Sender<ServerEvent>,
    pub(crate) rx_internal_server_commands: Receiver<InternalCommand>,
}

#[derive(Resource)]
pub struct ConnectionResources {
    pub(crate) active_connections: Arc<Mutex<HashMap<Uuid, ActiveConnection>>>,
}

pub struct ActiveConnection {
    pub(crate) operator_mode: OperatorMode,
    pub addr: String,
}

pub fn setup_connection_resources(mut commands: Commands) {
    let active_connections = Arc::new(Mutex::new(HashMap::new()));
    let client_channels = Arc::new(Mutex::new(HashMap::new()));

    let cloned_active_connections = active_connections.clone();
    let cloned_client_channels = client_channels.clone();

    let (tx_client_action_commands, rx_client_action_commands) = unbounded();
    let (tx_server_events, rx_server_events) = unbounded();
    let (tx_internal_server_commands, rx_internal_server_commands) = unbounded();
    let cloned_tx_server_events = tx_server_events.clone();
    let cloned_rx_internal_server_commands = rx_internal_server_commands.clone();

    IoTaskPool::get()
        .spawn(async move {
            setup_connection_handler(
                tx_client_action_commands,
                rx_server_events,
                tx_internal_server_commands,
                cloned_active_connections,
                cloned_client_channels,
            )
            .await
        })
        .detach();

    commands.insert_resource(ConnectionResources { active_connections });

    commands.insert_resource(ClientActionCommandReceiver {
        rx_client_action_commands,
    });

    commands.insert_resource(ServerEventSender {
        tx_server_events: cloned_tx_server_events,
        rx_internal_server_commands: cloned_rx_internal_server_commands,
    });
}

async fn setup_connection_handler(
    client_action_command_sender: Sender<(Uuid, ClientActionCommand)>,
    server_event_receiver: Receiver<ServerEvent>,
    tx_internal_server_commands: Sender<InternalCommand>,
    active_connections: Arc<Mutex<HashMap<Uuid, ActiveConnection>>>,
    client_channels: Arc<Mutex<HashMap<Uuid, Sender<ServerEvent>>>>,
) {
    info!("Spinning up server");
    let cloned_client_channels = client_channels.clone();
    IoTaskPool::get()
        .spawn({
            info!("Spinning up broadcaster task");
            let cloned_client_channels = cloned_client_channels.clone();
            async move {
                while let Ok(event) = server_event_receiver.recv().await {
                    let channels = cloned_client_channels.lock().await;
                    for (uuid, tx) in channels.iter() {
                        if let Err(e) = tx.send(event.clone()).await {
                            error!("Failed to broadcast event to client {uuid}: {:?}", e);
                        }
                    }
                }
            }
        })
        .detach();

    let addr = SocketAddr::from(([127, 0, 0, 1], 12345));
    match TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("Server listening on 127.0.0.1:12345");
            loop {
                if let Ok((mut stream, addr)) = listener.accept().await {
                    info!("New connection from: {}", addr);

                    let hello = wait_for_hello(&mut stream).await;
                    info!("Received hello: {:?}", hello);

                    if let Some(client_hello) = hello {
                        match client_hello {
                            OperatorMode::Operator => {
                                let already_connected = {
                                    let map = active_connections.lock().await;
                                    map.values()
                                        .any(|conn| conn.operator_mode == OperatorMode::Operator)
                                };

                                if already_connected {
                                    error!(
                                        "An operator is already connected. Rejecting new operator connection from {}",
                                        addr
                                    );

                                    // Send rejection back to the client.
                                    let _ = send_bincode_message(
                                        &mut stream,
                                        &ServerEvent::Hello(HelloState::Rejected {
                                            reason: "Operator already connected".to_string(),
                                        }),
                                    )
                                    .await;

                                    continue;
                                } else {
                                    // Send acceptance back to the client.
                                    let _ = send_bincode_message(
                                        &mut stream,
                                        &ServerEvent::Hello(HelloState::Accepted),
                                    )
                                    .await;
                                }

                                let (tx, rx) = unbounded();
                                let addr_str = addr.to_string();
                                let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, addr_str.as_bytes());

                                {
                                    cloned_client_channels.lock().await.insert(uuid, tx);
                                }

                                {
                                    let mut map = active_connections.lock().await;
                                    map.insert(
                                        uuid,
                                        ActiveConnection {
                                            addr: addr_str,
                                            operator_mode: OperatorMode::Operator,
                                        },
                                    );
                                }

                                if let Err(e) = tx_internal_server_commands
                                    .send(OperatorConnected { id: uuid })
                                    .await
                                {
                                    error!("[Operator] Failed to send internal command: {:?}", e);
                                }

                                let tx = client_action_command_sender.clone();

                                let cloned_tx_internal_server_commands =
                                    tx_internal_server_commands.clone();

                                let cloned_active_connections = active_connections.clone();

                                IoTaskPool::get()
                                    .spawn(async move {
                                        handle_operator_client(
                                            uuid,
                                            stream,
                                            tx,
                                            cloned_tx_internal_server_commands,
                                            rx,
                                            cloned_active_connections,
                                        )
                                        .await
                                    })
                                    .detach();
                            }
                            OperatorMode::DashboardViewer => {
                                // Send acceptance back to the client.
                                let _ = send_bincode_message(
                                    &mut stream,
                                    &ServerEvent::Hello(HelloState::Accepted),
                                )
                                .await;

                                let (tx, rx) = unbounded();
                                let addr_str = addr.to_string();
                                let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, addr_str.as_bytes());

                                {
                                    cloned_client_channels.lock().await.insert(uuid, tx);
                                }

                                {
                                    let mut map = active_connections.lock().await;
                                    map.insert(
                                        uuid,
                                        ActiveConnection {
                                            addr: addr_str,
                                            operator_mode: OperatorMode::DashboardViewer,
                                        },
                                    );
                                }

                                if let Err(e) = tx_internal_server_commands
                                    .send(DashboardViewerConnected { id: uuid })
                                    .await
                                {
                                    error!(
                                        "[DashboardViewer] Failed to send internal command: {:?}",
                                        e
                                    );
                                }

                                let cloned_tx_internal_server_commands =
                                    tx_internal_server_commands.clone();

                                let cloned_active_connections = active_connections.clone();

                                IoTaskPool::get()
                                    .spawn(async move {
                                        handle_dashboard_viewer_client(
                                            uuid,
                                            stream,
                                            cloned_tx_internal_server_commands,
                                            rx,
                                            cloned_active_connections,
                                        )
                                        .await
                                    })
                                    .detach();
                            }
                        }
                    } else {
                        error!("Received None hello");
                    }
                }

                sleep(Duration::from_millis(SLEEP_TIME_FOR_TARGET_TICK)).await;
            }
        }
        Err(e) => {
            error!("Could not bind to port {}: {:?}", addr, e);
        }
    }
}

#[instrument(skip_all, fields(client_id = %uuid))]
async fn handle_operator_client(
    uuid: Uuid,
    mut stream: TcpStream,
    tx_client_action_commands: Sender<(Uuid, ClientActionCommand)>,
    tx_internal_server_commands: Sender<InternalCommand>,
    rx_server_events: Receiver<ServerEvent>,
    active_connections: Arc<Mutex<HashMap<Uuid, ActiveConnection>>>,
) {
    loop {
        futures::select! {
            maybe_server_event = rx_server_events.recv().fuse() => {
                if let Ok(server_event) = maybe_server_event {
                    if let Err(e) = send_bincode_message(&mut stream, &server_event).await {
                       error!("[Operator] Failed to send broadcast to client: {:?}", e);
                    }
                }
            },

            client_message = read_bincode_message::<ClientMessage>(&mut stream).fuse() => {
                match client_message {
                    Ok(ClientMessage::ClientActionCommand(client_action_command)) => {
                        info!("[Operator] Received action command from client: {:?}", client_action_command);
                        tx_client_action_commands.send((uuid, client_action_command)).await.expect("Failed to forward command");
                    }
                    Ok(ClientMessage::Hello{ .. }) => {
                        warn!("[Operator] Received another client hello, this is weird.");
                    }
                    Err(e) => {
                        error!("[Operator] Client disconnected or error reading from stream: {:?}", e);

                        {
                            let mut map = active_connections.lock().await;
                            map.remove(&uuid);
                        }

                        if let Err(e) = tx_internal_server_commands.send(
                            InternalCommand::OperatorDisconnected { id: uuid }
                        ).await {
                            error!("[Operator] Failed to send PlayerDisconnected command: {:?}", e);
                        }

                        break;
                    }
                }
            }
        }
    }
}

#[instrument(skip_all, fields(client_id = %uuid))]
async fn handle_dashboard_viewer_client(
    uuid: Uuid,
    mut stream: TcpStream,
    tx_internal_server_commands: Sender<InternalCommand>,
    rx_server_events: Receiver<ServerEvent>,
    active_connections: Arc<Mutex<HashMap<Uuid, ActiveConnection>>>,
) {
    let mut length_buf = [0; 4];

    loop {
        futures::select! {
            maybe_server_event = rx_server_events.recv().fuse() => {
                if let Ok(server_event) = maybe_server_event {
                    if let Err(e) = send_bincode_message(&mut stream, &server_event).await {
                       error!("[DashboardViewer] Failed to send broadcast to client: {:?}", e);
                    }
                }
            },
            maybe_length_buf = stream.read_exact(&mut length_buf).fuse() => {
                match maybe_length_buf {
                    Ok(_) => {}
                    Err(e) => {
                        error!("[DashboardViewer] Dashboard Viewer disconnected or error reading from stream: {:?}", e);

                        {
                            let mut map = active_connections.lock().await;
                            map.remove(&uuid);
                        }

                        if let Err(e) = tx_internal_server_commands.send(
                            InternalCommand::DashboardViewerDisconnected { id: uuid }
                        ).await {
                            error!("[DashboardViewer] Failed to send PlayerDisconnected command: {:?}", e);
                        }
                        break;
                    }
                }
            }
        }
    }
}

async fn wait_for_hello(stream: &mut TcpStream) -> Option<OperatorMode> {
    match read_bincode_message::<ClientMessage>(stream).await {
        Ok(ClientMessage::Hello { mode }) => {
            info!("Received hello message from client");
            Some(mode)
        }
        Ok(other) => {
            warn!("Expected hello message, but received: {:?}", other);
            None
        }
        Err(e) => {
            error!("Failed to deserialize hello message: {:?}", e);
            None
        }
    }
}

async fn read_bincode_message<T: serde::de::DeserializeOwned>(
    stream: &mut TcpStream,
) -> anyhow::Result<T> {
    let mut length_buf = [0u8; 4];
    stream.read_exact(&mut length_buf).await?;
    let length = (&length_buf[..]).read_u32::<BigEndian>()? as usize;
    let mut buf = vec![0u8; length];
    stream.read_exact(&mut buf).await?;
    let message = bincode::deserialize(&buf)?;
    Ok(message)
}

async fn send_bincode_message<T: serde::Serialize>(
    stream: &mut TcpStream,
    value: &T,
) -> anyhow::Result<()> {
    let serialized = bincode::serialize(value)?;
    let length = serialized.len() as u32;

    let mut message = Vec::with_capacity(4 + serialized.len());
    message.write_u32::<BigEndian>(length)?;
    message.extend_from_slice(&serialized);

    stream.write_all(&message).await?;
    Ok(())
}
