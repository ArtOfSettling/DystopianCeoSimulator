use crate::internal_commands::InternalCommand;
use crate::systems::InternalCommandReceiver;
use crate::{GameServiceResource, Instances};
use bevy::prelude::{Res, ResMut};
use log::{debug, error};
use shared::ServerEvent;
use tracing::info;

pub fn process_internal_commands(
    mut instances: ResMut<Instances>,
    game_service: ResMut<GameServiceResource>,
    rx_internal_command: Res<InternalCommandReceiver>,
) {
    while let Ok(internal_command) = rx_internal_command.rx_internal_commands.try_recv() {
        info!(
            "Server has internal command for processing {:?}",
            internal_command
        );
        match internal_command {
            InternalCommand::Connected {
                client_info,
                operator_mode,
                tx_to_clients,
                rx_from_clients,
            } => {
                instances
                    .active_connections
                    .insert(client_info.id, client_info.clone());

                if instances
                    .active_instances
                    .contains_key(&client_info.game_id)
                {
                    debug!(
                        "Client id: {:?} connected to server in mode: {:?}, game: {:?} in memory",
                        client_info.id, operator_mode, client_info.game_id
                    );
                    let instance = instances
                        .active_instances
                        .get_mut(&client_info.game_id)
                        .unwrap();
                    instance.needs_broadcast = true;
                } else {
                    debug!("Client connected to server, game not in memory");
                    instances.add_new_instance(
                        &client_info.game_id,
                        tx_to_clients,
                        rx_from_clients,
                    );
                    let instance = instances
                        .active_instances
                        .get_mut(&client_info.game_id)
                        .unwrap();
                    instance.needs_broadcast = true;
                }
            }
            InternalCommand::Disconnected { id, game_id } => {
                instances.active_connections.remove(&id);

                if instances.active_instances.contains_key(&game_id) {
                    debug!(
                        "Client id: {:?} disconnected from server, game: {:?} in memory",
                        id, game_id
                    );
                    let instance = instances.active_instances.get_mut(&game_id).unwrap();
                    instance.needs_broadcast = true;
                } else {
                    debug!("Client disconnected from server, game not in memory");
                }
            }
            InternalCommand::CreateGame {
                client_id,
                game_name,
            } => {
                let game_service = game_service.game_service.clone();
                let tx_to_clients = instances
                    .active_connections
                    .get(&client_id)
                    .map(|c| c.sender.clone());

                async_std::task::spawn(async move {
                    match game_service.create_game(game_name.clone()).await {
                        Ok(game_metadata) => {
                            info!("Game created successfully: {:?}", game_metadata.id);
                            if let Some(tx) = tx_to_clients {
                                let _ = tx
                                    .send(ServerEvent::GameCreated {
                                        game_id: game_metadata.id,
                                        game_name: game_name.clone(),
                                    })
                                    .await;
                            }
                        }
                        Err(e) => {
                            error!("Failed to create game: {:?}", e);
                            if let Some(tx) = tx_to_clients {
                                let _ = tx
                                    .send(ServerEvent::GameCreationFailed {
                                        game_name: game_name.clone(),
                                        reason: e.to_string(),
                                    })
                                    .await;
                            }
                        }
                    }
                });
            }
        }
    }
}
