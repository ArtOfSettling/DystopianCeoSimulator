mod client_args;
pub mod game_data;
pub mod history_data;
pub mod resources;

use bevy::utils::HashMap;
pub use client_args::*;
pub use game_data::*;
pub use history_data::*;
pub use resources::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum OperatorMode {
    #[default]
    Operator,
    DashboardViewer,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Hello {
        requested_game_id: Uuid,
        mode: OperatorMode,
    },
    ClientActionCommand {
        requested_game_id: Uuid,
        command: ClientActionCommand,
    },
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

    Hello(HelloState),

    FullState(GameState),
    HistoryState(HistoryState),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HelloState {
    Accepted,
    Rejected { reason: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientActionCommand {
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
