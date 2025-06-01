mod redrive_event_logs;
mod create_empty_world_state;
mod process_broadcast_world_state;
pub(crate) mod process_clear_needs_state_update;
pub(crate) mod process_company_updates;
mod process_commands;
mod process_events;
mod process_internal_commands;
pub(crate) mod process_organization_updates;
mod process_print_active_connections;
mod setup_command_log;
mod setup_connection_resources;
mod setup_event_log;

use crate::{GameClientInternalEvent, Instance};
pub use redrive_event_logs::*;
pub use create_empty_world_state::*;
pub use process_broadcast_world_state::*;
pub use process_commands::*;
pub use process_events::*;
pub use process_internal_commands::*;
pub use process_print_active_connections::*;
use serde::{Deserialize, Serialize};
pub use setup_command_log::*;
pub use setup_connection_resources::*;
pub use setup_event_log::*;
use shared::{
    ClientActionCommand, CompanyHistory, Employment, InternalEvent, MAX_HISTORY_POINTS,
    OrganizationHistory, OrganizationRole, PlayerHistory,
};
use std::collections::VecDeque;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub fn apply_event(internal_event: &InternalEvent, instance: &mut Instance) {
    match internal_event {
        InternalEvent::RemoveEmployedStatus { employee_id } => {
            if let Some(entity) = instance
                .instance_game
                .game_state
                .entities
                .get_mut(employee_id)
            {
                entity.employment = None;
            }
        }

        InternalEvent::AddEmployedStatus {
            organization_id,
            employee_id,
        } => {
            if let Some(entity) = instance
                .instance_game
                .game_state
                .entities
                .get_mut(employee_id)
            {
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
            if let Some(player) = instance.instance_game.game_state.players.first_mut() {
                player.perception.reputation -= amount;
            }
        }

        InternalEvent::IncrementReputation { amount } => {
            if let Some(player) = instance.instance_game.game_state.players.first_mut() {
                player.perception.reputation += amount;
            }
        }

        InternalEvent::DecrementMoney { amount } => {
            if let Some(player) = instance.instance_game.game_state.players.first_mut() {
                player.financials.actual_cash -= amount;
            }
        }

        InternalEvent::IncrementMoney { amount } => {
            if let Some(player) = instance.instance_game.game_state.players.first_mut() {
                player.financials.actual_cash += amount;
            }
        }

        InternalEvent::IncrementEmployeeSatisfaction {
            employee_id,
            amount,
        } => {
            if let Some(entity) = instance
                .instance_game
                .game_state
                .entities
                .get_mut(employee_id)
            {
                if let Some(employment) = &mut entity.employment {
                    employment.satisfaction += amount;
                }
            }
        }

        InternalEvent::IncrementOrgPublicOpinion {
            organization_id,
            amount,
        } => {
            if let Some(organization) = instance
                .instance_game
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
            if let Some(organization) = instance
                .instance_game
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
            if let Some(entity) = instance
                .instance_game
                .game_state
                .entities
                .get_mut(employee_id)
            {
                if let Some(employment) = &mut entity.employment {
                    employment.salary += amount;
                }
            }
        }

        InternalEvent::SetOrgVp {
            organization_id,
            employee_id,
        } => {
            if let Some(organization) = instance
                .instance_game
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
            if let Some(entity) = instance
                .instance_game
                .game_state
                .entities
                .get_mut(employee_id)
            {
                if let Some(employment) = &mut entity.employment {
                    employment.role = *new_role;
                }
            }
        }

        InternalEvent::SetOrgFinancials {
            organization_id,
            financials,
        } => {
            if let Some(organization) = instance
                .instance_game
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
            if let Some(organization) = instance
                .instance_game
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
            if let Some(organization) = instance
                .instance_game
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
            if let Some(organization) = instance
                .instance_game
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
            if let Some(company) = instance
                .instance_game
                .game_state
                .companies
                .get_mut(company_id)
            {
                company.financials = financials.clone();
            }
        }

        InternalEvent::AppendHistoryPoint {
            new_player_history_points,
            new_company_history_points,
            new_organization_history_points,
        } => {
            for (player_id, history_point) in new_player_history_points {
                let player_history = instance
                    .instance_game
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
                let company_history = instance
                    .instance_game
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
                let org_history = instance
                    .instance_game
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
            instance.instance_game.game_state.week += 1;
            instance.needs_broadcast = true;
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggedCommand {
    pub version: u32,
    pub timestamp_epoch_millis: u64,
    pub source_client_id: Uuid,
    pub game_id: Uuid,
    pub command: ClientActionCommand,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggedEvent {
    pub version: u32,
    pub timestamp_epoch_millis: u64,
    pub game_id: Uuid,
    pub event: InternalEvent,
}

pub fn write_event_to_log_stream(
    event_log: &mut EventLog,
    game_id: &Uuid,
    event: GameClientInternalEvent,
) {
    let logged = LoggedEvent {
        version: 1,
        timestamp_epoch_millis: current_millis(),
        game_id: event.game_id,
        event: event.internal_event.clone(),
    };

    let writer = event_log.writer.get_writer(game_id);
    match serde_json::to_string(&logged) {
        Ok(serialized) => {
            if let Err(e) = writeln!(writer, "{}", serialized) {
                eprintln!("Failed to write to log file: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to serialize LoggedEvent: {}", e);
        }
    }

    if let Err(e) = writer.flush() {
        eprintln!("Failed to flush log file: {}", e);
    }
}

pub fn write_command_to_log_stream(
    command_log: &mut CommandLog,
    game_id: &Uuid,
    source_client_id: &Uuid,
    command: ClientActionCommand,
) {
    let logged = LoggedCommand {
        version: 1,
        timestamp_epoch_millis: current_millis(),
        source_client_id: *source_client_id,
        game_id: *game_id,
        command: command.clone(),
    };

    let writer = command_log.writer.get_writer(game_id);
    match serde_json::to_string(&logged) {
        Ok(serialized) => {
            if let Err(e) = writeln!(writer, "{}", serialized) {
                eprintln!("Failed to write to log file: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to serialize LoggedCommand: {}", e);
        }
    }

    if let Err(e) = writer.flush() {
        eprintln!("Failed to flush log file: {}", e);
    }
}

fn current_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}
