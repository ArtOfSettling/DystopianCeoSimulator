use bevy::prelude::Commands;
use input_api::PendingPlayerInputAction;
use shared::PendingPlayerAction;

pub fn setup_pending_player_action(mut commands: Commands) {
    commands.insert_resource(PendingPlayerAction(None));
    commands.insert_resource(PendingPlayerInputAction(None));
}
