use crate::systems::ClientCommandSender;
use bevy::prelude::{Res, ResMut};
use shared::{ClientMessage, PendingPlayerAction};
use tracing::info;

pub fn send_client_commands(
    channel: Res<ClientCommandSender>,
    mut pending_player_action: ResMut<PendingPlayerAction>,
) {
    if pending_player_action.0.is_none() {
        return;
    }

    let player_action = pending_player_action.0.clone().unwrap();
    info!(
        "Sending player_input_action Command: {:?}",
        pending_player_action
    );

    let _ = &channel
        .tx_client_commands
        .try_send(ClientMessage::ClientActionCommand(player_action));

    pending_player_action.0 = None;
}
