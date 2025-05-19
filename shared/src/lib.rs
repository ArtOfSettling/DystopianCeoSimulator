pub mod game_data;
pub mod history_data;
pub mod resources;

use bevy::prelude::Resource;
use bevy::utils::HashMap;
pub use game_data::*;
pub use history_data::*;
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
        amount: i16,
    },
    DecrementMoney {
        amount: i32,
    },
    IncrementEmployeeSatisfaction {
        employee_id: Uuid,
        amount: u16,
    },
    IncrementOrgPublicOpinion {
        organization_id: Uuid,
        amount: i16,
    },
    IncrementOrgReputation {
        organization_id: Uuid,
        amount: i16,
    },
    IncrementSalary {
        employee_id: Uuid,
        amount: u16,
    },
    IncrementReputation {
        amount: i16,
    },
    IncrementMoney {
        amount: i32,
    },
    SetOrgVp {
        organization_id: Uuid,
        employee_id: Option<Uuid>,
    },
    SetOrgFinancials {
        organization_id: Uuid,
        financials: Financials,
    },
    SetOrgInitiatives {
        organization_id: Uuid,
        initiatives: Vec<Initiative>,
    },
    SetOrgPublicOpinion {
        organization_id: Uuid,
        perception: Perception,
    },
    SetOrgBudget {
        organization_id: Uuid,
        budget: Budget,
    },
    SetOrganizationRole {
        employee_id: Uuid,
        new_role: OrganizationRole,
    },
    SetCompanyFinancials {
        company_id: Uuid,
        financials: Financials,
    },
    AppendHistoryPoint {
        new_player_history_points: HashMap<Uuid, HistoryPoint>,
        new_organization_history_points: HashMap<Uuid, HistoryPoint>,
        new_company_history_points: HashMap<Uuid, HistoryPoint>,
    },
    AdvanceWeek,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerEvent {
    None,

    FullState(GameState),
    HistoryState(HistoryState),
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
        amount: u16,
    },
    LaunchPRCampaign,
    DoNothing,
    PromoteToVp {
        organization_id: Uuid,
        employee_id: Uuid,
    },
    UpdateBudget {
        organization_id: Uuid,
        organization_budget: Budget,
    },
}

#[derive(Resource, Default, Debug)]
pub struct PendingPlayerAction(pub Option<PlayerAction>);
