use async_channel::{Receiver, Sender, bounded};
use async_std::io::ReadExt;
use async_std::net::TcpStream;
use bevy::prelude::{Commands, Resource};
use bevy::tasks::IoTaskPool;
use bevy::tasks::futures_lite::AsyncWriteExt;
use bincode;
use futures::future::Either;
use futures::{future, pin_mut};
use shared::{ClientCommand, ServerEvent};
use tracing::{debug, error, info, instrument, warn};

#[derive(Resource)]
pub struct ServerEventsReceiver {
    pub(crate) rx_server_events: Receiver<ServerEvent>,
}

#[derive(Resource)]
pub struct ClientCommandSender {
    pub(crate) tx_client_commands: Sender<ClientCommand>,
}

pub fn setup_connection_resources(mut commands: Commands) {
    let (tx_client_commands, rx_client_commands) = bounded(32);
    let (tx_server_events, rx_server_events) = bounded(32);

    IoTaskPool::get()
        .spawn(async move {
            info!("Connecting to server");
            let stream = TcpStream::connect("127.0.0.1:12345").await.unwrap();
            info!("Server listening on 127.0.0.1:12345");
            handle_server(stream, tx_server_events, rx_client_commands).await
        })
        .detach();

    commands.insert_resource(ClientCommandSender { tx_client_commands });
    commands.insert_resource(ServerEventsReceiver { rx_server_events })
}

#[instrument(skip(stream, tx_server_commands, rx_client_commands))]
async fn handle_server(
    mut stream: TcpStream,
    tx_server_commands: Sender<ServerEvent>,
    rx_client_commands: Receiver<ClientCommand>,
) {
    let mut buf = vec![0; 1024];

    loop {
        let future1 = async { rx_client_commands.recv().await };

        let future2 = async { stream.read(&mut buf).await };

        pin_mut!(future1);
        pin_mut!(future2);

        let res = future::select(future1, future2).await;
        match res {
            Either::Left((maybe_command, _)) => match maybe_command {
                Ok(client_command) => {
                    info!("Sending client command to server: {:?}", client_command);
                    let serialized = bincode::serialize(&client_command).unwrap();
                    if let Err(e) = stream.write_all(&serialized).await {
                        error!("Failed to send broadcast to server: {:?}", e);
                    }
                }
                Err(e) => {
                    error!("Some Error: {:?}", e);
                }
            },
            Either::Right((maybe_stream_buff, _)) => match maybe_stream_buff {
                Ok(n) if n > 0 => {
                    let event: ServerEvent = bincode::deserialize(&buf[..n]).unwrap();
                    debug!("Received event from server: {:?}", event);
                    tx_server_commands
                        .send(event)
                        .await
                        .expect("Failed to forward command");
                }
                Ok(_) => {
                    warn!("Server closed the connection");
                    break;
                }
                Err(e) => {
                    error!("Error reading from server: {:?}", e);
                    break;
                }
            },
        }
    }
}
