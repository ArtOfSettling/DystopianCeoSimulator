pub mod components;
pub mod resources;

use bevy::prelude::Resource;
pub use components::*;
pub use resources::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientCommand {
    PlayerAction(PlayerAction),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerEvent {
    None,

    FullState(GameStateSnapshot),
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct GameStateSnapshot {
    pub money: f64,
    pub reputation: i32,
    pub employees: Vec<EmployeeSnapshot>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmployeeSnapshot {
    pub id: Uuid,
    pub name: String,
    pub satisfaction: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlayerAction {
    FireEmployee(Uuid),
    GiveRaise(Uuid, f64),
    LaunchPRCampaign,
    DoNothing,
}

#[derive(Resource, Default, Debug)]
pub struct PendingPlayerAction(pub Option<PlayerAction>);
