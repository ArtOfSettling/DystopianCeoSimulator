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
pub enum InternalEvent {
    SetEmployeeStatus {
        target_id: Uuid,
        status: EmploymentStatus,
    },
    DecrementReputation {
        amount: u32,
    },
    DecrementMoney {
        amount: u32,
    },
    IncrementEmployeeSatisfaction {
        target_id: Uuid,
        amount: u32,
    },
    IncrementSalary {
        target_id: Uuid,
        amount: u32,
    },
    IncrementReputation {
        amount: u32,
    },
    IncrementMoney {
        amount: u32,
    },
    RemoveOrgVp {
        target_id: Uuid,
    },
    AdvanceWeek,
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
    pub organizations: Vec<OrganizationSnapshot>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationSnapshot {
    pub id: Uuid,
    pub name: String,
    pub vp: Option<Uuid>,
    pub employees: Vec<EmployeeSnapshot>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmployeeSnapshot {
    pub id: Uuid,
    pub name: String,
    pub satisfaction: i32,
    pub employment_status: EmploymentStatus,
    pub salary: i32,
    pub role: String,
    pub organization_id: Option<Uuid>,
    pub org_role: Option<OrgRole>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlayerAction {
    FireEmployee(Uuid),
    GiveRaise(Uuid, u32),
    LaunchPRCampaign,
    DoNothing,
}

#[derive(Resource, Default, Debug)]
pub struct PendingPlayerAction(pub Option<PlayerAction>);
