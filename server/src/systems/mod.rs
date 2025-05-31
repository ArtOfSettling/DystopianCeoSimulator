mod process_broadcast_world_state;
pub(crate) mod process_clear_needs_state_update;
mod process_client_commands;
mod process_command_log;
pub(crate) mod process_company_updates;
mod process_event_log;
mod process_fan_out_commands;
mod process_fan_out_events;
mod process_internal_commands;
mod process_internal_events;
pub(crate) mod process_organization_updates;
mod process_print_active_connections;
mod setup_command_log;
mod setup_connection_resources;
mod setup_event_log;
mod setup_fan_out_commands;
mod setup_fan_out_events;
mod setup_redrive_command_log;
mod setup_redrive_event_log;
mod setup_world_state;

use crate::NeedsWorldBroadcast;
pub use process_broadcast_world_state::*;
pub use process_client_commands::*;
pub use process_command_log::*;
pub use process_event_log::*;
pub use process_fan_out_commands::*;
pub use process_fan_out_events::*;
pub use process_internal_commands::*;
pub use process_internal_events::*;
pub use process_print_active_connections::*;
pub use setup_command_log::*;
pub use setup_connection_resources::*;
pub use setup_event_log::*;
pub use setup_fan_out_commands::*;
pub use setup_fan_out_events::*;
pub use setup_redrive_command_log::*;
pub use setup_redrive_event_log::*;
pub use setup_world_state::*;
use shared::{
    CompanyHistory, Employment, InternalEvent, MAX_HISTORY_POINTS, OrganizationHistory,
    OrganizationRole, PlayerHistory, ServerGameState, ServerHistoryState,
};
use std::collections::VecDeque;

pub fn apply_event(
    internal_event: &InternalEvent,
    server_game_state: &mut ServerGameState,
    server_history_state: &mut ServerHistoryState,
    needs_world_broadcast: &mut NeedsWorldBroadcast,
) {
    match internal_event {
        InternalEvent::RemoveEmployedStatus { employee_id } => {
            if let Some(entity) = server_game_state.game_state.entities.get_mut(employee_id) {
                entity.employment = None;
            }
        }

        InternalEvent::AddEmployedStatus {
            organization_id,
            employee_id,
        } => {
            if let Some(entity) = server_game_state.game_state.entities.get_mut(employee_id) {
                entity.employment = Some(Employment {
                    organization_id: *organization_id,
                    role: OrganizationRole::SalesRep,
                    employee_flags: vec![],
                    level: 10_000,
                    salary: 7_000,
                    satisfaction: 80,
                    productivity: 80,
                });
            }
        }

        InternalEvent::DecrementReputation { amount } => {
            if let Some(player) = server_game_state.game_state.players.first_mut() {
                player.perception.reputation -= amount;
            }
        }

        InternalEvent::IncrementReputation { amount } => {
            if let Some(player) = server_game_state.game_state.players.first_mut() {
                player.perception.reputation += amount;
            }
        }

        InternalEvent::DecrementMoney { amount } => {
            if let Some(player) = server_game_state.game_state.players.first_mut() {
                player.financials.actual_cash -= amount;
            }
        }

        InternalEvent::IncrementMoney { amount } => {
            if let Some(player) = server_game_state.game_state.players.first_mut() {
                player.financials.actual_cash += amount;
            }
        }

        InternalEvent::IncrementEmployeeSatisfaction {
            employee_id,
            amount,
        } => {
            if let Some(entity) = server_game_state.game_state.entities.get_mut(employee_id) {
                if let Some(employment) = &mut entity.employment {
                    employment.satisfaction += amount;
                }
            }
        }

        InternalEvent::IncrementOrgPublicOpinion {
            organization_id,
            amount,
        } => {
            if let Some(organization) = server_game_state
                .game_state
                .organizations
                .get_mut(organization_id)
            {
                organization.perception.public_opinion += amount;
            }
        }

        InternalEvent::IncrementOrgReputation {
            organization_id,
            amount,
        } => {
            if let Some(organization) = server_game_state
                .game_state
                .organizations
                .get_mut(organization_id)
            {
                organization.perception.reputation += amount;
            }
        }

        InternalEvent::IncrementSalary {
            employee_id,
            amount,
        } => {
            if let Some(entity) = server_game_state.game_state.entities.get_mut(employee_id) {
                if let Some(employment) = &mut entity.employment {
                    employment.salary += amount;
                }
            }
        }

        InternalEvent::SetOrgVp {
            organization_id,
            employee_id,
        } => {
            if let Some(organization) = server_game_state
                .game_state
                .organizations
                .get_mut(organization_id)
            {
                organization.vp = *employee_id;
            }
        }

        InternalEvent::SetOrganizationRole {
            employee_id,
            new_role,
        } => {
            if let Some(entity) = server_game_state.game_state.entities.get_mut(employee_id) {
                if let Some(employment) = &mut entity.employment {
                    employment.role = *new_role;
                }
            }
        }

        InternalEvent::SetOrgFinancials {
            organization_id,
            financials,
        } => {
            if let Some(organization) = server_game_state
                .game_state
                .organizations
                .get_mut(organization_id)
            {
                organization.financials = financials.clone();
            }
        }

        InternalEvent::SetOrgInitiatives {
            organization_id,
            initiatives,
        } => {
            if let Some(organization) = server_game_state
                .game_state
                .organizations
                .get_mut(organization_id)
            {
                organization.initiatives = initiatives.clone();
            }
        }

        InternalEvent::SetOrgPublicOpinion {
            organization_id,
            perception,
        } => {
            if let Some(organization) = server_game_state
                .game_state
                .organizations
                .get_mut(organization_id)
            {
                organization.perception = perception.clone();
            }
        }

        InternalEvent::SetOrgBudget {
            organization_id,
            budget,
        } => {
            if let Some(organization) = server_game_state
                .game_state
                .organizations
                .get_mut(organization_id)
            {
                organization.budget = budget.clone();
            }
        }

        InternalEvent::SetCompanyFinancials {
            company_id,
            financials,
        } => {
            if let Some(company) = server_game_state.game_state.companies.get_mut(company_id) {
                company.financials = financials.clone();
            }
        }

        InternalEvent::AppendHistoryPoint {
            new_player_history_points,
            new_company_history_points,
            new_organization_history_points,
        } => {
            for (player_id, history_point) in new_player_history_points {
                let player_history = server_history_state
                    .history_state
                    .players
                    .entry(*player_id)
                    .or_insert_with(|| PlayerHistory {
                        recent_history: VecDeque::new(),
                    });
                player_history
                    .recent_history
                    .push_back(history_point.clone());
                if player_history.recent_history.iter().len() > MAX_HISTORY_POINTS {
                    player_history.recent_history.pop_front();
                }
            }

            for (company_id, history_point) in new_company_history_points {
                let company_history = server_history_state
                    .history_state
                    .companies
                    .entry(*company_id)
                    .or_insert_with(|| CompanyHistory {
                        recent_history: VecDeque::new(),
                    });
                company_history
                    .recent_history
                    .push_back(history_point.clone());
                if company_history.recent_history.iter().len() > MAX_HISTORY_POINTS {
                    company_history.recent_history.pop_front();
                }
            }

            for (organization_id, history_point) in new_organization_history_points {
                let org_history = server_history_state
                    .history_state
                    .organizations
                    .entry(*organization_id)
                    .or_insert_with(|| OrganizationHistory {
                        recent_history: VecDeque::new(),
                    });
                org_history.recent_history.push_back(history_point.clone());
                if org_history.recent_history.iter().len() > MAX_HISTORY_POINTS {
                    org_history.recent_history.pop_front();
                }
            }
        }

        InternalEvent::AdvanceWeek => {
            server_game_state.game_state.week += 1;
            needs_world_broadcast.0 = true;
        }
    }
}
