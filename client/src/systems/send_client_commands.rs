use crate::systems::ClientCommandSender;
use bevy::prelude::{Res, ResMut};
use shared::{ClientMessage, PendingPlayerAction};
use tracing::info;
use uuid::{Uuid, uuid};

pub const GAME_ID: Uuid = uuid!("f47ac10b-58cc-4372-a567-0e02b2c3d479");

pub fn send_client_commands(
    channel: Res<ClientCommandSender>,
    mut pending_player_action: ResMut<PendingPlayerAction>,
) {
    if pending_player_action.0.is_none() {
        return;
    }

    let client_action_command = pending_player_action.0.clone().unwrap();
    info!(
        "Sending player_input_action Command: {:?}",
        pending_player_action
    );

    let _ = &channel
        .tx_client_commands
        .try_send(ClientMessage::ClientActionCommand {
            requested_game_id: GAME_ID,
            command: client_action_command,
        });

    pending_player_action.0 = None;
}
