use crate::systems::ClientCommandSender;
use bevy::prelude::Res;
use input_api::InputResource;
use shared::ClientCommand;
use tracing::info;

pub fn send_client_commands(channel: Res<ClientCommandSender>, input_resource: Res<InputResource>) {
    if let Some(player_action) = input_resource.input_handler.get_player_action() {
        info!("Sending player_action Command: {:?}", player_action);
        let _ = &channel
            .tx_client_commands
            .try_send(ClientCommand::PlayerAction(player_action));
    }
}
