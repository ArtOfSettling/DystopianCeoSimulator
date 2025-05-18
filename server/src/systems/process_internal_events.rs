use crate::NeedsWorldBroadcast;
use crate::systems::FanOutEventReceiver;
use bevy::prelude::{Res, ResMut};
use shared::{
    CompanyHistory, Employment, InternalEvent, MAX_HISTORY_POINTS, OrganizationHistory,
    OrganizationRole, PlayerHistory, ServerGameState, ServerHistoryState,
};
use std::collections::VecDeque;
use tracing::info;

pub fn process_internal_events(
    fan_out_event_receiver: Res<FanOutEventReceiver>,
    mut server_game_state: ResMut<ServerGameState>,
    mut server_history_state: ResMut<ServerHistoryState>,
    mut needs_world_broadcast: ResMut<NeedsWorldBroadcast>,
) {
    while let Ok(internal_event) = fan_out_event_receiver.rx_fan_out_events.try_recv() {
        info!(
            "Server has internal event for processing {:?}",
            internal_event
        );

        match internal_event {
            InternalEvent::RemoveEmployedStatus { employee_id } => {
                if let Some(entity) = server_game_state.game_state.entities.get_mut(&employee_id) {
                    entity.employment = None;
                }
            }

            InternalEvent::AddEmployedStatus {
                organization_id,
                employee_id,
            } => {
                if let Some(entity) = server_game_state.game_state.entities.get_mut(&employee_id) {
                    entity.employment = Some(Employment {
                        organization_id,
                        role: OrganizationRole::Worker,
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
                if let Some(entity) = server_game_state.game_state.entities.get_mut(&employee_id) {
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
                    .get_mut(&organization_id)
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
                    .get_mut(&organization_id)
                {
                    organization.perception.reputation += amount;
                }
            }

            InternalEvent::IncrementSalary {
                employee_id,
                amount,
            } => {
                if let Some(entity) = server_game_state.game_state.entities.get_mut(&employee_id) {
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
                    .get_mut(&organization_id)
                {
                    organization.vp = employee_id;
                }
            }
            
            InternalEvent::SetOrganizationRole {
                employee_id,
                new_role
            } => {
                if let Some(entity) = server_game_state.game_state.entities.get_mut(&employee_id) {
                    if let Some(employment) = &mut entity.employment {
                        employment.role = new_role;
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
                    .get_mut(&organization_id)
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
                    .get_mut(&organization_id)
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
                    .get_mut(&organization_id)
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
                    .get_mut(&organization_id)
                {
                    organization.budget = budget.clone();
                }
            }

            InternalEvent::SetCompanyFinancials {
                company_id,
                financials,
            } => {
                if let Some(company) = server_game_state.game_state.companies.get_mut(&company_id) {
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
                        .entry(player_id)
                        .or_insert_with(|| PlayerHistory {
                            recent_history: VecDeque::new(),
                        });
                    player_history.recent_history.push_back(history_point);
                    if player_history.recent_history.iter().len() > MAX_HISTORY_POINTS {
                        player_history.recent_history.pop_front();
                    }
                }

                for (company_id, history_point) in new_company_history_points {
                    let company_history = server_history_state
                        .history_state
                        .companies
                        .entry(company_id)
                        .or_insert_with(|| CompanyHistory {
                            recent_history: VecDeque::new(),
                        });
                    company_history.recent_history.push_back(history_point);
                    if company_history.recent_history.iter().len() > MAX_HISTORY_POINTS {
                        company_history.recent_history.pop_front();
                    }
                }

                for (organization_id, history_point) in new_organization_history_points {
                    let org_history = server_history_state
                        .history_state
                        .organizations
                        .entry(organization_id)
                        .or_insert_with(|| OrganizationHistory {
                            recent_history: VecDeque::new(),
                        });
                    org_history.recent_history.push_back(history_point);
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
}
