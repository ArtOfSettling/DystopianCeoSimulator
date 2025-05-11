use crate::internal_commands::InternalCommand;
use crate::internal_commands::InternalCommand::{PlayerConnected, PlayerDisconnected};
use async_channel::{Receiver, Sender, bounded};
use async_std::io::ReadExt;
use async_std::net::{TcpListener, TcpStream};
use async_std::task::sleep;
use bevy::prelude::{Commands, Resource};
use bevy::tasks::IoTaskPool;
use bevy::tasks::futures_lite::AsyncWriteExt;
use bincode;
use futures::FutureExt;
use shared::{ClientCommand, InternalEntity, ServerEvent};
use std::time::Duration;
use tracing::{error, info, instrument, warn};
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
    let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();

    info!("Server listening on 127.0.0.1:12345");
    loop {
        if let Ok((stream, addr)) = listener.accept().await {
            info!("New connection from: {}", addr);

            let addr_str = addr.to_string();
            let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, addr_str.as_bytes());
            tx_internal_server_commands
                .send(PlayerConnected(InternalEntity::new(uuid)))
                .await
                .unwrap();

            let tx = client_command_sender.clone();
            let rx = server_event_receiver.clone();

            let cloned_tx_internal_server_commands = tx_internal_server_commands.clone();
            IoTaskPool::get()
                .spawn(async move {
                    handle_client(uuid, stream, tx, cloned_tx_internal_server_commands, rx).await
                })
                .detach();
        }

        sleep(Duration::from_millis(SLEEP_TIME_FOR_TARGET_TICK)).await;
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
    let mut buf = vec![0; 1024];

    loop {
        futures::select! {
            maybe_server_command = rx_server_commands.recv().fuse() => {
                match maybe_server_command {
                    Ok(server_command) => {
                        let serialized = bincode::serialize(&server_command).unwrap();
                        if let Err(e) = stream.write_all(&serialized).await {
                            error!("Failed to send broadcast to client: {:?}", e);
                        }
                    }
                    Err(_) => {}
                }
            },

            maybe_stream_buff = stream.read(&mut buf).fuse() => {
                match maybe_stream_buff {
                    Ok(n) if n > 0 => {
                        let command: ClientCommand = bincode::deserialize(&buf[..n]).unwrap();
                        info!("Received command from client: {:?}", command);
                        tx_client_commands.send((uuid, command)).await.expect("Failed to forward command");
                    }
                    Ok(_) => {
                        warn!("Client closed the connection");

                        tx_internal_server_commands
                            .send(PlayerDisconnected(InternalEntity::new(uuid)))
                            .await
                            .unwrap();

                        break;
                    }
                    Err(e) => {
                        error!("Error reading from server: {:?}", e);
                        break;
                    }
                }
            }
        }
    }
}
