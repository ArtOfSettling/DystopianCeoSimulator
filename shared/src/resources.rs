use crate::{Financials, OrgBudget};
use bevy::prelude::Resource;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Resource)]
pub struct Company {
    pub reputation: i32,
    pub public_opinion: i32,
    pub financials: Financials,
}

#[derive(Default, Resource)]
pub struct HistoricalData {
    pub org_history: HashMap<Uuid, Vec<OrganizationHistoryEntry>>,
}

#[derive(Clone)]
pub struct OrganizationHistoryEntry {
    pub week: i32,
    pub cash: i32,
    pub income: i32,
    pub expenses: i32,
    pub net_profit: i32,
    pub public_opinion: i32,
    pub reputation: i32,
    pub avg_employee_satisfaction: i32,
    pub budgets: OrgBudget,
}

#[derive(Resource, Default, Debug)]
pub enum MenuState {
    #[default]
    AwaitingInput,
    ProcessingAction,
    DisplayingSummary,
}

#[derive(Resource, Debug)]
pub enum PendingAction {
    FireEmployee(Uuid),
    GiveRaise(Uuid, f64),
    LaunchPRCampaign,
    DoNothing,
}
