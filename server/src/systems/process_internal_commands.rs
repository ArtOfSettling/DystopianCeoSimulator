use crate::internal_commands::InternalCommand;
use crate::systems::ServerEventSender;
use bevy::prelude::{Commands, Entity, Query, Res};
use shared::components::InternalEntity;
use shared::{Money, Player, Reputation, Week};
use tracing::info;

pub fn process_internal_commands(
    mut commands: Commands,
    player_query: Query<(
        Entity,
        &Player,
        &Money,
        &Reputation,
        &Week,
        Option<&InternalEntity>,
    )>,
    channel: Res<ServerEventSender>,
) {
    while let Ok(internal_command) = channel.rx_internal_server_commands.try_recv() {
        info!(
            "Server has internal command for processing {:?}",
            internal_command
        );
        match internal_command {
            InternalCommand::PlayerConnected(internal_entity) => {
                info!("attaching tracked_entity {:?}", internal_entity.clone());

                let (entity, _, _, _, _, _) = player_query.get_single().unwrap();
                info!(
                    "attaching tracked_entity {:?} to player",
                    internal_entity.clone()
                );

                commands.entity(entity).insert(internal_entity.clone());
            }
            InternalCommand::PlayerDisconnected(disconnected_internal_entity) => {
                info!(
                    "de-spawning player with uuid {:?} at (0, 0)",
                    disconnected_internal_entity
                );

                let (entity, _, _, _, _, _) = player_query.get_single().unwrap();

                commands.entity(entity).remove::<InternalEntity>();
            }
        }
    }
}
