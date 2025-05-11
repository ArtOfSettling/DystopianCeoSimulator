use crate::internal_commands::InternalCommand;
use crate::systems::ServerEventSender;
use bevy::prelude::{Commands, Entity, Query, Res};
use shared::ServerEvent::InitialWorldState;
use shared::{InternalEntity, Player, Position, WorldState};
use tracing::{debug, error, info};

pub fn process_internal_commands(
    mut commands: Commands,
    player_query: Query<(Entity, &Player, Option<&InternalEntity>, &Position)>,
    channel: Res<ServerEventSender>,
) {
    while let Ok(internal_command) = channel.rx_internal_server_commands.try_recv() {
        info!(
            "Server has internal command for processing {:?}",
            internal_command
        );
        match internal_command {
            InternalCommand::PlayerConnected(internal_entity) => {
                info!(
                    "attaching tracked_entity {:?}",
                    internal_entity.clone()
                );

                let (entity, _, _, position) = player_query.get_single().unwrap();
                info!(
                    "attaching tracked_entity {:?} to player at {:?}",
                    internal_entity.clone(),
                    position.clone()
                );

                commands
                    .entity(entity)
                    .insert(internal_entity.clone());

                match channel
                    .tx_server_events
                    .try_send(InitialWorldState(WorldState {
                        player_1_internal_entity: internal_entity.clone(),
                        player_1_position: position.clone(),
                    })) {
                    Ok(_) => {
                        debug!("Updated world state");
                    }
                    Err(e) => {
                        error!("Failed to update world state: {:?}", e);
                    }
                }
            }
            InternalCommand::PlayerDisconnected(disconnected_internal_entity) => {
                info!(
                    "de-spawning player with uuid {:?} at (0, 0)",
                    disconnected_internal_entity
                );

                let (entity, _, _, _) = player_query.get_single().unwrap();

                commands
                    .entity(entity)
                    .remove::<InternalEntity>();
            }
        }
    }
}
