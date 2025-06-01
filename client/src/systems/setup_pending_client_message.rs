use bevy::prelude::Commands;
use shared::PendingClientMessage;

pub fn setup_pending_client_message(mut commands: Commands) {
    commands.insert_resource(PendingClientMessage(None));
}
