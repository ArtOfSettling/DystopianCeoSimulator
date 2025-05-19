use crate::systems::FanOutClientCommandReceiver;
use crate::{InternalEventSender, NeedsStateUpdate};
use bevy::prelude::{Res, ResMut};
use bevy::utils::HashMap;
use shared::{
    ClientCommand, HistoryPoint, InternalEvent, OrganizationRole, PlayerAction, ServerGameState,
};
use tracing::info;
use uuid::Uuid;

pub fn process_client_commands(
    channel: Res<FanOutClientCommandReceiver>,
    mut needs_state_update: ResMut<NeedsStateUpdate>,
    internal_event_sender: Res<InternalEventSender>,
    server_game_state: Res<ServerGameState>,
) {
    while let Ok((_, client_command)) = channel.rx_fan_out_client_commands.try_recv() {
        needs_state_update.0 = true;
        info!(
            "Server has clients command for processing {:?}",
            client_command
        );
        match client_command {
            ClientCommand::PlayerAction(player_action) => match player_action {
                PlayerAction::FireEmployee { employee_id } => {
                    if let Some(employee) = server_game_state.game_state.entities.get(&employee_id)
                    {
                        info!("Firing employee: {}", employee.name);

                        let _ = internal_event_sender
                            .tx_internal_events
                            .try_send(InternalEvent::RemoveEmployedStatus { employee_id });

                        for (org_id, org) in &server_game_state.game_state.organizations {
                            if org.vp == Some(employee_id) {
                                let _ = internal_event_sender.tx_internal_events.try_send(
                                    InternalEvent::SetOrgVp {
                                        organization_id: *org_id,
                                        employee_id: None,
                                    },
                                );
                            }
                        }
                    }
                }

                PlayerAction::HireEmployee {
                    organization_id,
                    employee_id,
                } => {
                    if let Some((employee_id, employee)) = server_game_state
                        .game_state
                        .entities
                        .iter()
                        .find(|(entity_id, _)| **entity_id == employee_id)
                    {
                        info!("Hiring employee: {}", employee.name);
                        internal_event_sender
                            .tx_internal_events
                            .try_send(InternalEvent::AddEmployedStatus {
                                organization_id,
                                employee_id: *employee_id,
                            })
                            .unwrap();
                    }
                }

                PlayerAction::GiveRaise {
                    employee_id,
                    amount,
                } => {
                    if let Some((employee_id, employee)) = server_game_state
                        .game_state
                        .entities
                        .iter()
                        .find(|(entity_id, _)| **entity_id == employee_id)
                    {
                        info!("Growing employee: {}", employee.name);

                        internal_event_sender
                            .tx_internal_events
                            .try_send(InternalEvent::IncrementEmployeeSatisfaction {
                                employee_id: *employee_id,
                                amount: 1,
                            })
                            .unwrap();

                        internal_event_sender
                            .tx_internal_events
                            .try_send(InternalEvent::IncrementSalary {
                                employee_id: *employee_id,
                                amount,
                            })
                            .unwrap();
                    }
                }

                PlayerAction::LaunchPRCampaign => {
                    internal_event_sender
                        .tx_internal_events
                        .try_send(InternalEvent::IncrementReputation { amount: 1 })
                        .unwrap();

                    internal_event_sender
                        .tx_internal_events
                        .try_send(InternalEvent::DecrementMoney { amount: 1_000 })
                        .unwrap();
                }

                PlayerAction::DoNothing => {
                    info!("Player did nothing this turn.");
                }

                PlayerAction::PromoteToVp {
                    organization_id,
                    employee_id,
                } => {
                    internal_event_sender
                        .tx_internal_events
                        .try_send(InternalEvent::SetOrganizationRole {
                            employee_id,
                            new_role: OrganizationRole::VP,
                        })
                        .unwrap();

                    let existing_vp_id = server_game_state
                        .game_state
                        .organizations
                        .get(&organization_id)
                        .and_then(|org| org.vp);

                    if let Some(employee_id) = existing_vp_id {
                        internal_event_sender
                            .tx_internal_events
                            .try_send(InternalEvent::SetOrganizationRole {
                                employee_id,
                                new_role: OrganizationRole::HRManager,
                            })
                            .unwrap();
                    }

                    internal_event_sender
                        .tx_internal_events
                        .try_send(InternalEvent::SetOrgVp {
                            organization_id,
                            employee_id: Some(employee_id),
                        })
                        .unwrap();
                }

                PlayerAction::UpdateBudget {
                    organization_id,
                    organization_budget,
                } => internal_event_sender
                    .tx_internal_events
                    .try_send(InternalEvent::SetOrgBudget {
                        organization_id,
                        budget: organization_budget,
                    })
                    .unwrap(),
            },
        }

        let total_productivity: i32 = server_game_state
            .game_state
            .entities
            .values()
            .filter(|entity| entity.employment.is_some())
            .filter_map(|entity| entity.employment.as_ref().map(|e| e.satisfaction as i32))
            .sum();

        let total_expenses = server_game_state
            .game_state
            .entities
            .values()
            .filter(|entity| entity.employment.is_some())
            .filter_map(|entity| entity.employment.as_ref().map(|e| e.salary as i32))
            .sum();

        internal_event_sender
            .tx_internal_events
            .try_send(InternalEvent::IncrementMoney {
                amount: total_productivity,
            })
            .unwrap();

        internal_event_sender
            .tx_internal_events
            .try_send(InternalEvent::DecrementMoney {
                amount: total_expenses,
            })
            .unwrap();

        let mut new_player_history_points = HashMap::new();
        let mut new_company_history_points = HashMap::new();
        let mut new_organization_history_points = HashMap::new();

        for player in &server_game_state.game_state.players {
            let employees = server_game_state
                .game_state
                .entities
                .values()
                .filter_map(|entity| entity.employment.as_ref());

            let (total_satisfaction, employee_count) = employees
                .map(|e| e.satisfaction as u32)
                .fold((0, 0), |(sum, count), sat| (sum + sat, count + 1));

            let avg_employee_satisfaction = if employee_count > 0 {
                ((total_satisfaction * 10000) / employee_count).min(10000) as u16
            } else {
                0
            };

            let history_point = HistoryPoint {
                week: server_game_state.game_state.week,
                financials: player.financials.clone(),
                perception: player.perception.clone(),
                avg_employee_satisfaction,
            };

            new_player_history_points.insert(
                player.id.unwrap_or(Uuid::from_u128(987123564738)),
                history_point,
            );
        }

        for (company_id, company) in &server_game_state.game_state.companies {
            let (total_satisfaction, employee_count): (u32, u32) = server_game_state
                .game_state
                .entities
                .values()
                .filter_map(|entity| {
                    let employment = entity.employment.as_ref()?;
                    let organization = server_game_state
                        .game_state
                        .organizations
                        .get(&employment.organization_id)?;
                    if organization.company_relation.entity_id == *company_id {
                        Some((employment.satisfaction as u32, 1))
                    } else {
                        None
                    }
                })
                .fold((0, 0), |(sum, count), (satisfaction, one)| {
                    (sum + satisfaction, count + one)
                });

            let avg_employee_satisfaction: u16 = if employee_count > 0 {
                ((total_satisfaction * 10000) / employee_count).min(10000) as u16
            } else {
                0
            };

            let history_point = HistoryPoint {
                week: server_game_state.game_state.week,
                financials: company.financials.clone(),
                perception: company.perception.clone(),
                avg_employee_satisfaction,
            };

            new_company_history_points.insert(*company_id, history_point);
        }

        for (organization_id, organization) in &server_game_state.game_state.organizations {
            let (total_satisfaction, employee_count): (u32, u32) = server_game_state
                .game_state
                .entities
                .values()
                .filter_map(|entity| {
                    entity
                        .employment
                        .as_ref()
                        .filter(|e| e.organization_id == *organization_id)
                        .map(|e| (e.satisfaction as u32, 1))
                })
                .fold((0, 0), |(sum, count), (satisfaction, one)| {
                    (sum + satisfaction, count + one)
                });

            let avg_employee_satisfaction: u16 = if employee_count > 0 {
                ((total_satisfaction * 10000) / employee_count).min(10000) as u16
            } else {
                0
            };

            let history_point = HistoryPoint {
                week: server_game_state.game_state.week,
                financials: organization.financials.clone(),
                perception: organization.perception.clone(),
                avg_employee_satisfaction,
            };

            new_organization_history_points.insert(*organization_id, history_point);
        }

        internal_event_sender
            .tx_internal_events
            .try_send(InternalEvent::AppendHistoryPoint {
                new_player_history_points,
                new_organization_history_points,
                new_company_history_points,
            })
            .unwrap();

        internal_event_sender
            .tx_internal_events
            .try_send(InternalEvent::AdvanceWeek)
            .unwrap();
    }
}
