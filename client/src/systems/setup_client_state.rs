use bevy::prelude::*;
use renderer_api::{ClientGameState, ClientHistoryState};
use shared::HistoryState;

pub fn setup_world_state(mut commands: Commands) {
    commands.insert_resource(ClientGameState {
        week: 0,
        players: vec![],
        companies: Default::default(),
        organizations: Default::default(),
        entities: Default::default(),
        ordered_organizations_of_company: Default::default(),
        ordered_employees_of_organization: Default::default(),
        ordered_employees_of_company: Default::default(),
        ordered_unemployed_entities: vec![],
        ordered_pets_of_entity: Default::default(),
        ordered_children_of_entity: Default::default(),
    });
    commands.insert_resource(ClientHistoryState {
        history_state: HistoryState {
            players: Default::default(),
            organizations: Default::default(),
            companies: Default::default(),
        },
        player_order: vec![],
        organization_order: vec![],
        company_order: vec![],
    });
}
