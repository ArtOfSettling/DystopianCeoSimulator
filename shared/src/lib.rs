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
    RemoveEmployedStatus {
        employee_id: Uuid,
    },
    AddEmployedStatus {
        employee_id: Uuid,
        organization_id: Uuid,
    },
    DecrementReputation {
        amount: u32,
    },
    DecrementMoney {
        amount: u32,
    },
    IncrementEmployeeSatisfaction {
        employee_id: Uuid,
        amount: u32,
    },
    IncrementSalary {
        employee_id: Uuid,
        amount: u32,
    },
    IncrementReputation {
        amount: u32,
    },
    IncrementMoney {
        amount: u32,
    },
    RemoveOrgVp {
        organization_id: Uuid,
    },
    SetOrgVp {
        organization_id: Uuid,
        employee_id: Uuid,
    },
    AdvanceWeek,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerEvent {
    None,

    FullState(GameStateSnapshot),
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub enum UnemployedSnapshot {
    UnemployedAnimalSnapshot(AnimalSnapshot),
    UnemployedHumanSnapshot(HumanSnapshot),
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct GameStateSnapshot {
    pub week: u32,
    pub money: i32,
    pub reputation: i32,
    pub organizations: Vec<OrganizationSnapshot>,
    pub humans: Vec<HumanSnapshot>,
    pub pets: Vec<AnimalSnapshot>,
    pub unemployed: Vec<UnemployedSnapshot>,
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
    pub level: u32,
    pub satisfaction: i32,
    pub salary: i32,
    pub role: OrgRole,
    pub entity_type: EntityType,
    pub organization_id: Option<Uuid>,
    pub children_ids: Vec<Uuid>,
    pub pet_ids: Vec<Uuid>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HumanSnapshot {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnimalSnapshot {
    pub id: Uuid,
    pub name: String,
    pub entity_type: EntityType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlayerAction {
    FireEmployee {
        employee_id: Uuid,
    },
    HireEmployee {
        employee_id: Uuid,
        organization_id: Uuid,
    },
    GiveRaise {
        employee_id: Uuid,
        amount: u32,
    },
    LaunchPRCampaign,
    DoNothing,
    PromoteToVp {
        organization_id: Uuid,
        employee_id: Uuid,
    },
}

#[derive(Resource, Default, Debug)]
pub struct PendingPlayerAction(pub Option<PlayerAction>);
