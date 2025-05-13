use bevy::prelude::*;
use shared::GameStateSnapshot;

pub fn setup_world_state(mut commands: Commands) {
    commands.insert_resource(GameStateSnapshot {
        week: 1,
        money: 0,
        reputation: 0,
        employees: vec![],
    })
}
