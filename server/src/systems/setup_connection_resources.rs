use crate::internal_commands::InternalCommand;
use crate::internal_commands::InternalCommand::PlayerConnected;
use async_channel::{Receiver, Sender, bounded};
use async_std::io::ReadExt;
use async_std::net::{TcpListener, TcpStream};
use async_std::task::sleep;
use bevy::prelude::{Commands, Resource};
use bevy::tasks::IoTaskPool;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use futures::{AsyncWriteExt, FutureExt};
use shared::{ClientCommand, ServerEvent};
use std::net::SocketAddr;
use std::time::Duration;
use tracing::{error, info, instrument};
use uuid::Uuid;

const TARGET_SERVER_TICK: u64 = 32;
const SLEEP_TIME_FOR_TARGET_TICK: u64 = 1000 / TARGET_SERVER_TICK;

#[derive(Resource)]
pub struct ClientCommandReceiver {
    pub(crate) rx_client_commands: Receiver<(Uuid, ClientCommand)>,
}

#[derive(Resource)]
pub struct ServerEventSender {
    pub(crate) tx_server_events: Sender<ServerEvent>,
    pub(crate) rx_internal_server_commands: Receiver<InternalCommand>,
}

pub fn setup_connection_resources(mut commands: Commands) {
    let (tx_client_commands, rx_client_commands) = bounded(32);
    let (tx_server_events, rx_server_events) = bounded(32);
    let (tx_internal_server_commands, rx_internal_server_commands) = bounded(32);
    let cloned_tx_server_events = tx_server_events.clone();
    let cloned_rx_internal_server_commands = rx_internal_server_commands.clone();

    IoTaskPool::get()
        .spawn(async move {
            setup_connection_handler(
                tx_client_commands,
                rx_server_events,
                tx_internal_server_commands,
            )
            .await
        })
        .detach();

    commands.insert_resource(ClientCommandReceiver { rx_client_commands });

    commands.insert_resource(ServerEventSender {
        tx_server_events: cloned_tx_server_events,
        rx_internal_server_commands: cloned_rx_internal_server_commands,
    })
}

async fn setup_connection_handler(
    client_command_sender: Sender<(Uuid, ClientCommand)>,
    server_event_receiver: Receiver<ServerEvent>,
    tx_internal_server_commands: Sender<InternalCommand>,
) {
    info!("Spinning up server");
    let addr = SocketAddr::from(([127, 0, 0, 1], 12345));
    match TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("Server listening on 127.0.0.1:12345");
            loop {
                if let Ok((stream, addr)) = listener.accept().await {
                    info!("New connection from: {}", addr);

                    let addr_str = addr.to_string();
                    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, addr_str.as_bytes());
                    tx_internal_server_commands
                        .send(PlayerConnected { player_id: uuid })
                        .await
                        .unwrap();

                    let tx = client_command_sender.clone();
                    let rx = server_event_receiver.clone();

                    let cloned_tx_internal_server_commands = tx_internal_server_commands.clone();
                    IoTaskPool::get()
                        .spawn(async move {
                            handle_client(uuid, stream, tx, cloned_tx_internal_server_commands, rx)
                                .await
                        })
                        .detach();
                }

                sleep(Duration::from_millis(SLEEP_TIME_FOR_TARGET_TICK)).await;
            }
        }
        Err(e) => {
            error!("Could not bind to port {}: {:?}", addr, e);
        }
    }
}

#[instrument(skip(stream, tx_client_commands, rx_server_commands))]
async fn handle_client(
    uuid: Uuid,
    mut stream: TcpStream,
    tx_client_commands: Sender<(Uuid, ClientCommand)>,
    tx_internal_server_commands: Sender<InternalCommand>,
    rx_server_commands: Receiver<ServerEvent>,
) {
    let mut length_buf = [0; 4];

    loop {
        futures::select! {
            maybe_server_command = rx_server_commands.recv().fuse() => {
                if let Ok(server_command) = maybe_server_command {
                    let mut message = Vec::new();
                    let serialized = bincode::serialize(&server_command).unwrap();
                    // first write the buffer size
                    let length = serialized.len() as u32;
                    message.write_u32::<BigEndian>(length).unwrap();
                    // append the actual payload
                    message.extend_from_slice(&serialized);

                    // then write the buffer
                    if let Err(e) = stream.write_all(&message).await {
                        error!("Failed to send broadcast to client: {:?}", e);
                    }
                }
            },

            maybe_length_buf = stream.read_exact(&mut length_buf).fuse() => {
                match maybe_length_buf {
                    Ok(_) => {
                        // first read the buffer size
                        let length = (&length_buf[..]).read_u32::<BigEndian>().unwrap() as usize;

                        // then read the buffer
                        let mut msg_buf = vec![0; length];
                        stream.read_exact(&mut msg_buf).await.unwrap();

                        let client_command: ClientCommand = match bincode::deserialize(&msg_buf) {
                            Ok(msg) => msg,
                            Err(e) => {
                                error!("Deserialization failed: {}", e);
                                panic!("Deserialization failed: {}", e);
                            }
                        };

                        info!("Received command from client: {:?}", client_command);
                        tx_client_commands.send((uuid, client_command)).await.expect("Failed to forward command");
                    }
                    Err(e) => {
                        error!("Client disconnected or error reading from stream: {:?}", e);

                        // Send PlayerDisconnected internal command before exiting
                        if let Err(e) = tx_internal_server_commands.send(
                            InternalCommand::PlayerDisconnected { player_id: uuid }
                        ).await {
                            error!("Failed to send PlayerDisconnected command: {:?}", e);
                        }

                        break;
                    }
                }
            }
        }
    }
}
