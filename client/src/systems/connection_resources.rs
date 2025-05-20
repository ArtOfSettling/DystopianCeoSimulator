use async_channel::{Receiver, Sender, bounded};
use async_std::io::ReadExt;
use async_std::net::TcpStream;
use bevy::prelude::{Commands, Res, Resource};
use bevy::tasks::IoTaskPool;
use bevy::tasks::futures_lite::AsyncWriteExt;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use futures::FutureExt;
use shared::{ClientMessage, OperatorModeResource, ServerEvent};
use tracing::{error, info, instrument};

#[derive(Resource)]
pub struct ServerEventsReceiver {
    pub(crate) rx_server_events: Receiver<ServerEvent>,
}

#[derive(Resource)]
pub struct ClientCommandSender {
    pub(crate) tx_client_commands: Sender<ClientMessage>,
}

pub fn setup_connection_resources(
    operator_mode_resource: Res<OperatorModeResource>,
    mut commands: Commands,
) {
    let (tx_client_commands, rx_client_commands) = bounded(32);
    let (tx_server_events, rx_server_events) = bounded(32);
    let operator_mode = operator_mode_resource.operator_mode.clone();

    IoTaskPool::get()
        .spawn(async move {
            info!("Connecting to server");
            let mut stream = TcpStream::connect("127.0.0.1:12345").await.unwrap();

            let serialized = bincode::serialize(&ClientMessage::Hello {
                mode: operator_mode,
            })
            .unwrap();
            let mut msg = Vec::new();
            msg.write_u32::<BigEndian>(serialized.len() as u32).unwrap();
            msg.extend_from_slice(&serialized);
            stream.write_all(&msg).await.unwrap();

            info!("Connected and sent hello");

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
    rx_client_commands: Receiver<ClientMessage>,
) {
    let mut length_buf = [0; 4];

    loop {
        futures::select! {
            maybe_client_command = rx_client_commands.recv().fuse() => {
                if let Ok(client_command) = maybe_client_command {
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
            },

            maybe_length_buf = stream.read_exact(&mut length_buf).fuse() => {
                match maybe_length_buf {
                    Ok(_) => {
                        // first read the buffer size
                        let length = (&length_buf[..]).read_u32::<BigEndian>().unwrap();

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
