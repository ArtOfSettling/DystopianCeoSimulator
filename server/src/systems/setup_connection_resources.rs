use crate::GameClientActionCommand;
use crate::internal_commands::InternalCommand;
use async_channel::{Receiver, Sender, unbounded};
use async_std::io::ReadExt;
use async_std::net::{TcpListener, TcpStream};
use async_std::sync::Mutex;
use async_std::task::spawn;
use bevy::prelude::{Commands, Resource};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use futures::AsyncWriteExt;
use serde::{Deserialize, Serialize};
use shared::{ClientMessage, HelloState, OperatorMode, ServerEvent};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ClientInfo {
    pub id: Uuid,
    pub game_id: Uuid,
    pub addr: String,
    pub operator_mode: OperatorMode,
    pub sender: Sender<ServerEvent>,
}

#[derive(Resource)]
pub struct InternalCommandReceiver {
    pub rx_internal_commands: Receiver<InternalCommand>,
}

pub fn start_server_system(mut commands: Commands) {
    let connections = Arc::new(Mutex::new(HashMap::new()));
    let (tx_to_clients, rx_to_clients) = unbounded();
    let (tx_from_clients, rx_from_clients) = unbounded();
    let (tx_internal_commands, rx_internal_commands) = unbounded();

    commands.insert_resource(InternalCommandReceiver {
        rx_internal_commands: rx_internal_commands.clone(),
    });

    spawn(broadcast_loop(rx_to_clients, connections.clone()));
    spawn(connection_listener_loop(
        connections,
        tx_to_clients,
        tx_from_clients,
        rx_from_clients,
        tx_internal_commands,
    ));
}

async fn broadcast_loop(
    rx_to_clients: Receiver<ServerEvent>,
    clients: Arc<Mutex<HashMap<Uuid, ClientInfo>>>,
) {
    while let Ok(event) = rx_to_clients.recv().await {
        let all_clients = clients.lock().await;
        for (uuid, client_info) in all_clients.iter() {
            if let Err(e) = client_info.sender.send(event.clone()).await {
                error!("Failed to send to client {uuid}: {e:?}");
            }
        }
    }
}

async fn connection_listener_loop(
    clients: Arc<Mutex<HashMap<Uuid, ClientInfo>>>,
    tx_to_clients: Sender<ServerEvent>,
    tx_from_clients: Sender<GameClientActionCommand>,
    rx_from_clients: Receiver<GameClientActionCommand>,
    tx_internal_commands: Sender<InternalCommand>,
) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 12345));
    let listener = TcpListener::bind(addr).await.expect("Failed to bind TCP");
    info!("Server listening on {addr}");

    while let Ok((stream, addr)) = listener.accept().await {
        let tx_to_clients = tx_to_clients.clone();
        let tx_from_clients = tx_from_clients.clone();
        let tx_internal_commands = tx_internal_commands.clone();
        let rx_from_clients = rx_from_clients.clone();
        let clients = clients.clone();

        spawn(async move {
            handle_connection(
                stream,
                addr,
                clients,
                tx_to_clients,
                tx_from_clients,
                rx_from_clients,
                tx_internal_commands,
            )
            .await;
        });
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    addr: SocketAddr,
    clients: Arc<Mutex<HashMap<Uuid, ClientInfo>>>,
    tx_to_clients: Sender<ServerEvent>,
    tx_from_clients: Sender<GameClientActionCommand>,
    rx_from_clients: Receiver<GameClientActionCommand>,
    tx_internal_commands: Sender<InternalCommand>,
) {
    let client_message = read_bincode_message::<ClientMessage>(&mut stream).await;
    if let Ok(ClientMessage::Hello {
        requested_game_id,
        mode,
    }) = client_message
    {
        let client_id = Uuid::new_v4();

        if mode == OperatorMode::Operator {
            let locked = clients.lock().await;
            if locked.values().any(|c| {
                c.game_id == requested_game_id && c.operator_mode == OperatorMode::Operator
            }) {
                let _ = send_bincode_message(
                    &mut stream,
                    &ServerEvent::Hello(HelloState::Rejected {
                        reason: "Operator already connected to this game".to_string(),
                    }),
                )
                .await;
                return;
            }
        }

        let _ = send_bincode_message(&mut stream, &ServerEvent::Hello(HelloState::Accepted)).await;
        let (tx, rx) = unbounded();

        let client_info = ClientInfo {
            id: client_id,
            game_id: requested_game_id,
            operator_mode: mode.clone(),
            addr: addr.to_string(),
            sender: tx.clone(),
        };

        clients.lock().await.insert(client_id, client_info.clone());

        let cloned_tx_internal_commands = tx_internal_commands.clone();
        spawn(forward_to_client_loop(stream.clone(), rx));
        spawn(read_from_client_loop(
            stream,
            clients.clone(),
            client_id,
            requested_game_id,
            tx_from_clients,
            cloned_tx_internal_commands,
        ));

        tx_internal_commands
            .send(InternalCommand::Connected {
                client_info: client_info.clone(),
                operator_mode: mode.clone(),
                tx_to_clients,
                rx_from_clients,
            })
            .await
            .expect("Failed to send internal command");
    }
}

async fn forward_to_client_loop(mut stream: TcpStream, rx: Receiver<ServerEvent>) {
    while let Ok(event) = rx.recv().await {
        if let Err(e) = send_bincode_message(&mut stream, &event).await {
            error!("Failed to send event: {e:?}");
            break;
        }
    }
}

async fn read_from_client_loop(
    mut stream: TcpStream,
    clients: Arc<Mutex<HashMap<Uuid, ClientInfo>>>,
    uuid: Uuid,
    game_id: Uuid,
    tx: Sender<GameClientActionCommand>,
    tx_internal_commands: Sender<InternalCommand>,
) {
    loop {
        match read_bincode_message::<ClientMessage>(&mut stream).await {
            Ok(ClientMessage::CreateGame { game_name }) => {
                let _ = tx_internal_commands
                    .send(InternalCommand::CreateGame {
                        client_id: uuid,
                        game_name,
                    })
                    .await;
            }
            Ok(ClientMessage::ListGames) => {
                let _ = tx_internal_commands
                    .send(InternalCommand::ListGames { client_id: uuid })
                    .await;
            }
            Ok(ClientMessage::ClientActionCommand {
                requested_game_id,
                command,
            }) => {
                if tx
                    .send(GameClientActionCommand {
                        source_client_id: uuid,
                        game_id,
                        command,
                    })
                    .await
                    .is_err()
                {
                    info!("Client {uuid} disconnected");
                    let mut clients_guard = clients.lock().await;
                    clients_guard.remove(&uuid);
                    drop(clients_guard);
                    let _ = tx_internal_commands
                        .send(InternalCommand::Disconnected {
                            id: uuid,
                            game_id: requested_game_id,
                        })
                        .await;
                    break;
                }
            }
            Ok(_) => {}
            Err(e) => {
                info!("Client {uuid} disconnected or errored: {e}");
                let mut clients_guard = clients.lock().await;
                clients_guard.remove(&uuid);
                drop(clients_guard);
                let _ = tx_internal_commands
                    .send(InternalCommand::Disconnected { id: uuid, game_id })
                    .await;
                break;
            }
        }
    }
}

async fn read_bincode_message<T: for<'a> Deserialize<'a>>(
    stream: &mut TcpStream,
) -> anyhow::Result<T> {
    let mut length_buf = [0u8; 4];
    if let Err(e) = stream.read_exact(&mut length_buf).await {
        return Err(anyhow::anyhow!("Client disconnected or stream error: {e}"));
    }

    let length = (&length_buf[..]).read_u32::<BigEndian>()? as usize;
    let mut buf = vec![0u8; length];

    if let Err(e) = stream.read_exact(&mut buf).await {
        return Err(anyhow::anyhow!("Client disconnected or stream error: {e}"));
    }

    let value = bincode::deserialize(&buf)?;
    Ok(value)
}

async fn send_bincode_message<T: Serialize>(stream: &mut TcpStream, msg: &T) -> anyhow::Result<()> {
    let serialized = bincode::serialize(msg)?;
    let mut buf = vec![0u8; 4 + serialized.len()];
    (&mut buf[..4]).write_u32::<BigEndian>(serialized.len() as u32)?;
    buf[4..].copy_from_slice(&serialized);
    stream.write_all(&buf).await?;
    Ok(())
}
