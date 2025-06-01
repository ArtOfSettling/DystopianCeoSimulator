use crate::systems::ClientCommandSender;
use bevy::prelude::{Res, ResMut};
use shared::{ClientMessage, PendingClientMessage, PendingPlayerAction};
use tracing::info;
use uuid::{Uuid, uuid};

pub const GAME_ID: Uuid = uuid!("f47ac10b-58cc-4372-a567-0e02b2c3d479");

pub fn send_client_commands(
    channel: Res<ClientCommandSender>,
    pending_player_action: ResMut<PendingPlayerAction>,
    pending_client_message: ResMut<PendingClientMessage>,
) {
    process_pending_client_message(&channel, pending_client_message);
    process_pending_player_action(&channel, pending_player_action);
}

fn process_pending_client_message(
    channel: &Res<ClientCommandSender>,
    mut pending_client_message: ResMut<PendingClientMessage>,
) {
    if pending_client_message.0.is_none() {
        return;
    }

    let client_message = pending_client_message.0.clone().unwrap();
    info!("Sending client_message Command: {:?}", client_message);

    let _ = &channel.tx_client_commands.try_send(client_message);

    pending_client_message.0 = None;
}

fn process_pending_player_action(
    channel: &Res<ClientCommandSender>,
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
