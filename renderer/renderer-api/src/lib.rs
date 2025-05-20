use bevy::app::AppExit;
use bevy::prelude::{EventWriter, ResMut, Resource};
use input_api::PendingPlayerInputAction;
use shared::{Company, Entity, HistoryState, Organization, PendingPlayerAction, Player};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Resource)]
pub struct ClientGameState {
    pub week: u16,
    pub players: Vec<Player>,
    pub companies: HashMap<Uuid, Company>,
    pub organizations: HashMap<Uuid, Organization>,
    pub entities: HashMap<Uuid, Entity>,

    // For Predictable ordering
    pub ordered_organizations_of_company: HashMap<Uuid, Vec<Uuid>>,
    pub ordered_employees_of_organization: HashMap<Uuid, Vec<Uuid>>,
    pub ordered_employees_of_company: HashMap<Uuid, Vec<Uuid>>,
    pub ordered_unemployed_entities: Vec<Uuid>,
    pub ordered_pets_of_entity: HashMap<Uuid, Vec<Uuid>>,
    pub ordered_children_of_entity: HashMap<Uuid, Vec<Uuid>>,
}

#[derive(Resource)]
pub struct ClientHistoryState {
    pub history_state: HistoryState,

    // For Predictable ordering
    pub player_order: Vec<Uuid>,
    pub organization_order: Vec<Uuid>,
    pub company_order: Vec<Uuid>,
}

#[derive(Resource)]
pub struct RendererResource {
    pub renderer: Box<dyn Renderer + Send + Sync>,
}

impl RendererResource {
    pub fn new(renderer: Box<dyn Renderer + Send + Sync>) -> RendererResource {
        Self { renderer }
    }
}

pub trait Renderer {
    fn render(
        &mut self,
        game_state: &ClientGameState,
        history_state: &ClientHistoryState,
        pending_player_input_action: ResMut<PendingPlayerInputAction>,
        pending_player_action: ResMut<PendingPlayerAction>,
        exit_writer: EventWriter<AppExit>,
    );
}
