use crate::systems::ClientCommandReceiver;
use bevy::prelude::{Query, Res};
use shared::{ClientCommand, Player, PlayerAction, Position};
use tracing::info;

pub fn process_client_commands(
    channel: Res<ClientCommandReceiver>,
    mut player_query: Query<(&Player, &mut Position)>,
) {
    while let Ok(client_command) = channel.rx_client_commands.try_recv() {
        info!(
            "Server has clients command for processing {:?}",
            client_command
        );
        match client_command {
            ClientCommand::PlayerAction(player_action) => match player_action {
                PlayerAction::MovePlayerLocalUp => {
                    for (_player, mut position) in &mut player_query {
                        position.y += 1;
                    }
                }
                PlayerAction::MovePlayerLocalRight => {
                    for (_player, mut position) in &mut player_query {
                        position.x += 1;
                    }
                }
                PlayerAction::MovePlayerLocalDown => {
                    for (_player, mut position) in &mut player_query {
                        position.y -= 1;
                    }
                }
                PlayerAction::MovePlayerLocalLeft => {
                    for (_player, mut position) in &mut player_query {
                        position.x -= 1;
                    }
                }

                PlayerAction::MovePlayerRoomUp => {}
                PlayerAction::MovePlayerRoomRight => {}
                PlayerAction::MovePlayerRoomDown => {}
                PlayerAction::MovePlayerRoomLeft => {}
                PlayerAction::GoDownStairs => {}
                PlayerAction::GoUpStairs => {}
                PlayerAction::ChatMessage(_) => {}
            },
        }
    }
}
