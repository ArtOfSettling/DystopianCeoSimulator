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
    pub week: u32,
    pub money: i32,
    pub reputation: i32,
    pub employees: Vec<EmployeeSnapshot>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmployeeSnapshot {
    pub id: Uuid,
    pub name: String,
    pub satisfaction: i32,
    pub salary: i32,
    pub employment_status: EmploymentStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlayerAction {
    FireEmployee(Uuid),
    GiveRaise(Uuid, i32),
    LaunchPRCampaign,
    DoNothing,
}

#[derive(Resource, Default, Debug)]
pub struct PendingPlayerAction(pub Option<PlayerAction>);
