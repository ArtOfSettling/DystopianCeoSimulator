use crate::systems::{CommandLog, write_command_to_log_stream};
use crate::{GameClientActionCommand, GameClientInternalEvent, Instance, Instances};
use bevy::prelude::ResMut;
use bevy::utils::HashMap;
use shared::{ClientActionCommand, HistoryPoint, InternalEvent, OrganizationRole};
use tracing::{debug, info};
use uuid::Uuid;

// Provides fan-out capabilities. Consumes events via the receiver and fans them out
// to all who need to listen.
pub fn process_commands(mut command_log: ResMut<CommandLog>, mut instances: ResMut<Instances>) {
    for (game_id, instance) in instances.active_instances.iter_mut() {
        while let Ok(client_action_command) = instance.rx_from_client.try_recv() {
            debug!("Writing command: {:?}", client_action_command);
            write_command_to_log_stream(
                &mut command_log,
                game_id,
                &client_action_command.source_client_id,
                client_action_command.command.clone(),
            );

            debug!("Processing command: {:?}", client_action_command);
            process_command(game_id, client_action_command, instance);
        }
    }
}

fn process_command(
    game_id: &Uuid,
    client_action_command: GameClientActionCommand,
    instance: &mut Instance,
) {
    let client_action_command = client_action_command.command;

    instance.needs_state_update = true;

    info!(
        "Server has clients command for processing {:?}",
        client_action_command
    );

    match client_action_command {
        ClientActionCommand::FireEmployee { employee_id } => {
            if let Some(employee) = instance.instance_game.game_state.entities.get(&employee_id) {
                info!("Firing employee: {}", employee.name);

                let _ = instance
                    .tx_internal_events
                    .try_send(GameClientInternalEvent {
                        game_id: *game_id,
                        internal_event: InternalEvent::RemoveEmployedStatus { employee_id },
                    });

                for (org_id, org) in &instance.instance_game.game_state.organizations {
                    if org.vp == Some(employee_id) {
                        let _ = instance
                            .tx_internal_events
                            .try_send(GameClientInternalEvent {
                                game_id: *game_id,
                                internal_event: InternalEvent::SetOrgVp {
                                    organization_id: *org_id,
                                    employee_id: None,
                                },
                            });
                    }
                }
            }
        }

        ClientActionCommand::HireEmployee {
            organization_id,
            employee_id,
        } => {
            if let Some((employee_id, employee)) = instance
                .instance_game
                .game_state
                .entities
                .iter()
                .find(|(entity_id, _)| **entity_id == employee_id)
            {
                info!("Hiring employee: {}", employee.name);
                instance
                    .tx_internal_events
                    .try_send(GameClientInternalEvent {
                        game_id: *game_id,
                        internal_event: InternalEvent::AddEmployedStatus {
                            organization_id,
                            employee_id: *employee_id,
                        },
                    })
                    .unwrap();
            }
        }

        ClientActionCommand::GiveRaise {
            employee_id,
            amount,
        } => {
            if let Some((employee_id, employee)) = instance
                .instance_game
                .game_state
                .entities
                .iter()
                .find(|(entity_id, _)| **entity_id == employee_id)
            {
                info!("Growing employee: {}", employee.name);

                instance
                    .tx_internal_events
                    .try_send(GameClientInternalEvent {
                        game_id: *game_id,
                        internal_event: InternalEvent::IncrementEmployeeSatisfaction {
                            employee_id: *employee_id,
                            amount: 1,
                        },
                    })
                    .unwrap();

                instance
                    .tx_internal_events
                    .try_send(GameClientInternalEvent {
                        game_id: *game_id,
                        internal_event: InternalEvent::IncrementSalary {
                            employee_id: *employee_id,
                            amount,
                        },
                    })
                    .unwrap();
            }
        }

        ClientActionCommand::LaunchPRCampaign => {
            instance
                .tx_internal_events
                .try_send(GameClientInternalEvent {
                    game_id: *game_id,
                    internal_event: InternalEvent::IncrementReputation { amount: 1 },
                })
                .unwrap();

            instance
                .tx_internal_events
                .try_send(GameClientInternalEvent {
                    game_id: *game_id,
                    internal_event: InternalEvent::DecrementMoney { amount: 1_000 },
                })
                .unwrap();
        }

        ClientActionCommand::DoNothing => {
            info!("Player did nothing this turn.");
        }

        ClientActionCommand::PromoteToVp {
            organization_id,
            employee_id,
        } => {
            instance
                .tx_internal_events
                .try_send(GameClientInternalEvent {
                    game_id: *game_id,
                    internal_event: InternalEvent::SetOrganizationRole {
                        employee_id,
                        new_role: OrganizationRole::VP,
                    },
                })
                .unwrap();

            let existing_vp_id = instance
                .instance_game
                .game_state
                .organizations
                .get(&organization_id)
                .and_then(|org| org.vp);

            if let Some(employee_id) = existing_vp_id {
                instance
                    .tx_internal_events
                    .try_send(GameClientInternalEvent {
                        game_id: *game_id,
                        internal_event: InternalEvent::SetOrganizationRole {
                            employee_id,
                            new_role: OrganizationRole::HRManager,
                        },
                    })
                    .unwrap();
            }

            instance
                .tx_internal_events
                .try_send(GameClientInternalEvent {
                    game_id: *game_id,
                    internal_event: InternalEvent::SetOrgVp {
                        organization_id,
                        employee_id: Some(employee_id),
                    },
                })
                .unwrap();
        }

        ClientActionCommand::UpdateBudget {
            organization_id,
            organization_budget,
        } => instance
            .tx_internal_events
            .try_send(GameClientInternalEvent {
                game_id: *game_id,
                internal_event: InternalEvent::SetOrgBudget {
                    organization_id,
                    budget: organization_budget,
                },
            })
            .unwrap(),
    }

    let total_productivity: i32 = instance
        .instance_game
        .game_state
        .entities
        .values()
        .filter(|entity| entity.employment.is_some())
        .filter_map(|entity| entity.employment.as_ref().map(|e| e.satisfaction as i32))
        .sum();

    let total_expenses = instance
        .instance_game
        .game_state
        .entities
        .values()
        .filter(|entity| entity.employment.is_some())
        .filter_map(|entity| entity.employment.as_ref().map(|e| e.salary as i32))
        .sum();

    instance
        .tx_internal_events
        .try_send(GameClientInternalEvent {
            game_id: *game_id,
            internal_event: InternalEvent::IncrementMoney {
                amount: total_productivity,
            },
        })
        .unwrap();

    instance
        .tx_internal_events
        .try_send(GameClientInternalEvent {
            game_id: *game_id,
            internal_event: InternalEvent::DecrementMoney {
                amount: total_expenses,
            },
        })
        .unwrap();

    let mut new_player_history_points = HashMap::new();
    let mut new_company_history_points = HashMap::new();
    let mut new_organization_history_points = HashMap::new();

    for player in &instance.instance_game.game_state.players {
        let employees = instance
            .instance_game
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
            week: instance.instance_game.game_state.week,
            financials: player.financials.clone(),
            perception: player.perception.clone(),
            avg_employee_satisfaction,
        };

        new_player_history_points.insert(
            player.id.unwrap_or(Uuid::from_u128(987123564738)),
            history_point,
        );
    }

    for (company_id, company) in &instance.instance_game.game_state.companies {
        let (total_satisfaction, employee_count): (u32, u32) = instance
            .instance_game
            .game_state
            .entities
            .values()
            .filter_map(|entity| {
                let employment = entity.employment.as_ref()?;
                let organization = instance
                    .instance_game
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
            week: instance.instance_game.game_state.week,
            financials: company.financials.clone(),
            perception: company.perception.clone(),
            avg_employee_satisfaction,
        };

        new_company_history_points.insert(*company_id, history_point);
    }

    for (organization_id, organization) in &instance.instance_game.game_state.organizations {
        let (total_satisfaction, employee_count): (u32, u32) = instance
            .instance_game
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
            week: instance.instance_game.game_state.week,
            financials: organization.financials.clone(),
            perception: organization.perception.clone(),
            avg_employee_satisfaction,
        };

        new_organization_history_points.insert(*organization_id, history_point);
    }

    instance
        .tx_internal_events
        .try_send(GameClientInternalEvent {
            game_id: *game_id,
            internal_event: InternalEvent::AppendHistoryPoint {
                new_player_history_points,
                new_organization_history_points,
                new_company_history_points,
            },
        })
        .unwrap();

    instance
        .tx_internal_events
        .try_send(GameClientInternalEvent {
            game_id: *game_id,
            internal_event: InternalEvent::AdvanceWeek,
        })
        .unwrap();
}
