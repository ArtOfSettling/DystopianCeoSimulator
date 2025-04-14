use crate::internal_commands::InternalCommand;
use crate::systems::ServerEventSender;
use bevy::prelude::{Commands, Entity, Query, Res};
use shared::{Player, Position};
use tracing::info;

pub fn process_internal_commands(
    mut commands: Commands,
    player_query: Query<(Entity, &Player, &Position)>,
    channel: Res<ServerEventSender>,
) {
    while let Ok(internal_command) = channel.rx_internal_server_commands.try_recv() {
        info!(
            "Server has internal command for processing {:?}",
            internal_command
        );
        match internal_command {
            InternalCommand::PlayerConnected(uuid) => {
                info!("spawning player with uuid {:?} at (0, 0)", uuid);
                let spawn_position = Position::new(0, 0);
                let player = Player::new(uuid);
                info!("actually spawning");
                commands.spawn((player, spawn_position));
            }
            InternalCommand::PlayerDisconnected(uuid) => {
                info!("de-spawning player with uuid {:?} at (0, 0)", uuid);
                player_query.iter().for_each(|(entity, player, _position)| {
                    if player.id == uuid {
                        info!("actually de-spawning");
                        commands.entity(entity).despawn()
                    }
                });
            }
        }
    }
}
