use crate::NeedsWorldBroadcast;
use crate::internal_commands::InternalCommand;
use crate::systems::ServerEventSender;
use bevy::prelude::{Res, ResMut};
use shared::ServerGameState;
use tracing::info;

pub fn process_internal_commands(
    mut needs_broadcast: ResMut<NeedsWorldBroadcast>,
    channel: Res<ServerEventSender>,
    mut server_game_state: ResMut<ServerGameState>,
) {
    if server_game_state.game_state.players.len() > 1 {
        panic!("More than one player in the world, this is weird");
    }

    if let Some(player) = server_game_state.game_state.players.first_mut() {
        while let Ok(internal_command) = channel.rx_internal_server_commands.try_recv() {
            info!(
                "Server has internal command for processing {:?}",
                internal_command
            );

            match internal_command {
                InternalCommand::OperatorConnected { id } => {
                    info!("attaching Operator with uuid {:?}", id);
                    player.id = Some(id);
                }
                InternalCommand::OperatorDisconnected { id } => {
                    info!("de-spawning Operator with uuid {:?}", id);
                    player.id = None;
                }
                InternalCommand::DashboardViewerConnected { id } => {
                    info!("de-spawning Dashboard Viewer with uuid {:?}", id);
                }
                InternalCommand::DashboardViewerDisconnected { id } => {
                    info!("de-spawning Dashboard Viewer with uuid {:?}", id);
                }
            }

            needs_broadcast.0 = true;
        }
    } else {
        panic!("Not a single player in the world, this is weird")
    }
}
