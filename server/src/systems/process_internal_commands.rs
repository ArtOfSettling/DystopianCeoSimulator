use crate::internal_commands::InternalCommand;
use crate::systems::InternalCommandReceiver;
use crate::{ActiveConnection, Instances};
use bevy::prelude::{Res, ResMut};
use log::debug;
use tracing::info;

pub fn process_internal_commands(
    mut instances: ResMut<Instances>,
    rx_internal_command: Res<InternalCommandReceiver>,
) {
    while let Ok(internal_command) = rx_internal_command.rx_internal_commands.try_recv() {
        info!(
            "Server has internal command for processing {:?}",
            internal_command
        );
        match internal_command {
            InternalCommand::Connected {
                id,
                addr,
                game_id,
                operator_mode,
                tx_to_clients,
                rx_from_clients,
            } => {
                instances.active_connections.insert(
                    id,
                    ActiveConnection {
                        game_id,
                        operator_mode: operator_mode.clone(),
                        addr: addr.to_string(),
                    },
                );

                if instances.active_instances.contains_key(&game_id) {
                    debug!(
                        "Client id: {:?} connected to server in mode: {:?}, game: {:?} in memory",
                        id, operator_mode, game_id
                    );
                    let instance = instances.active_instances.get_mut(&game_id).unwrap();
                    instance.needs_broadcast = true;
                } else {
                    debug!("Client connected to server, game not in memory");
                    instances.add_new_instance(&game_id, tx_to_clients, rx_from_clients);
                    let instance = instances.active_instances.get_mut(&game_id).unwrap();
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
        }
    }
}
