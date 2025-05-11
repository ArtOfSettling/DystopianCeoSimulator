use crate::systems::{
    FanOutClientCommandReceiver
};
use bevy::prelude::{Query, Res};
use shared::{ClientCommand, InternalEntity, Player, PlayerAction, Position};
use tracing::info;

pub fn process_client_commands(
    channel: Res<FanOutClientCommandReceiver>,
    mut player_query: Query<(&Player, Option<&InternalEntity>, &mut Position)>,
) {
    while let Ok((_, client_command)) = channel.rx_fan_out_client_commands.try_recv() {
        info!(
            "Server has clients command for processing {:?}",
            client_command
        );
        match client_command {
            ClientCommand::PlayerAction(player_action) => match player_action {
                PlayerAction::MovePlayerLocalUp => {
                    for (_player, _internal_entity, mut position) in &mut player_query {
                        position.y += 1;
                    }
                }
                PlayerAction::MovePlayerLocalRight => {
                    for (_player, _internal_entity, mut position) in &mut player_query {
                        position.x += 1;
                    }
                }
                PlayerAction::MovePlayerLocalDown => {
                    for (_player, _internal_entity, mut position) in &mut player_query {
                        position.y -= 1;
                    }
                }
                PlayerAction::MovePlayerLocalLeft => {
                    for (_player, _internal_entity, mut position) in &mut player_query {
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
