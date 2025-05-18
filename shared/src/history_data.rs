use crate::{Financials, Perception};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

pub const MAX_HISTORY_POINTS: usize = 50;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryState {
    pub players: HashMap<Uuid, PlayerHistory>,
    pub organizations: HashMap<Uuid, OrganizationHistory>,
    pub companies: HashMap<Uuid, CompanyHistory>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationHistory {
    pub recent_history: VecDeque<HistoryPoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompanyHistory {
    pub recent_history: VecDeque<HistoryPoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerHistory {
    pub recent_history: VecDeque<HistoryPoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryPoint {
    pub week: u16,
    pub financials: Financials,
    pub perception: Perception,
    pub avg_employee_satisfaction: u16,
}
