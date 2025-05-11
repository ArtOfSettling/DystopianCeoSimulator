use bevy::prelude::Commands;
use tracing::info;
use shared::{Player, Position};

pub fn setup_world_state(mut commands: Commands) {
    let spawn_position = Position::new(0, 0);
    let player = Player {};
    info!("spawning player");
    commands.spawn((player, spawn_position.clone()));
}
