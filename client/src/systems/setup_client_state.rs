use bevy::prelude::*;
use shared::{Financials, GameStateSnapshot};

pub fn setup_world_state(mut commands: Commands) {
    commands.insert_resource(GameStateSnapshot {
        week: 1,
        reputation: 0,
        financials: Financials {
            this_weeks_income: 0,
            this_weeks_expenses: 0,
            this_weeks_net_profit: 0,
            actual_cash: 0,
        },
        organizations: vec![],
        pets: vec![],
        humans: vec![],
        unemployed: vec![],
    })
}
