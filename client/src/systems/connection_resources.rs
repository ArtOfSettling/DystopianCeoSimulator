use async_channel::{Receiver, Sender, bounded};
use async_std::io::ReadExt;
use async_std::net::TcpStream;
use bevy::prelude::{Commands, Resource};
use bevy::tasks::IoTaskPool;
use bevy::tasks::futures_lite::AsyncWriteExt;
use bincode;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use futures::FutureExt;
use shared::{ClientCommand, ServerEvent};
use tracing::{error, info, instrument};

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
    let mut length_buf = [0; 4];

    loop {
        futures::select! {
            maybe_client_command = rx_client_commands.recv().fuse() => {
                match maybe_client_command {
                    Ok(client_command) => {
                        let mut message = Vec::new();
                        let serialized = bincode::serialize(&client_command).unwrap();
                        // first write the buffer size
                        let length = serialized.len() as u32;
                        message.write_u32::<BigEndian>(length).unwrap();
                        // append the actual payload
                        message.extend_from_slice(&serialized);

                        // then write the buffer
                        if let Err(e) = stream.write_all(&message).await {
                            error!("Failed to send to server: {:?}", e);
                        }
                    }
                    Err(_) => {}
                }
            },

            maybe_length_buf = stream.read_exact(&mut length_buf).fuse() => {
                match maybe_length_buf {
                    Ok(_) => {
                        // first read the buffer size
                        let length = (&length_buf[..]).read_u32::<BigEndian>().unwrap() as u32;

                        // then read the buffer
                        let mut msg_buf = vec![0; length as usize];
                        stream.read_exact(&mut msg_buf).await.unwrap();

                        let server_event: ServerEvent = match bincode::deserialize(&msg_buf) {
                            Ok(msg) => msg,
                            Err(e) => {
                                error!("Deserialization failed: {}", e);
                                panic!("Deserialization failed: {}", e);
                            }
                        };

                        info!("Received command from server: {:?}", server_event);
                        tx_server_commands
                            .send(server_event)
                            .await
                            .expect("Failed to forward event");
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
