use crate::InternalEventSender;
use crate::systems::FanOutClientCommandReceiver;
use bevy::prelude::{Query, Res};
use shared::{
    ClientCommand, Employee, EmploymentStatus, InternalEvent, Organization, PlayerAction,
    Productivity, Salary, Satisfaction,
};
use tracing::info;

pub fn process_client_commands(
    channel: Res<FanOutClientCommandReceiver>,
    internal_event_sender: Res<InternalEventSender>,
    mut employee_query: Query<(
        &mut Employee,
        &mut Salary,
        &mut Satisfaction,
        &mut Productivity,
    )>,
    mut organization_query: Query<&Organization>,
) {
    while let Ok((_, client_command)) = channel.rx_fan_out_client_commands.try_recv() {
        info!(
            "Server has clients command for processing {:?}",
            client_command
        );
        match client_command {
            ClientCommand::PlayerAction(player_action) => match player_action {
                PlayerAction::FireEmployee(target_id) => {
                    for (employee, _, _, _) in employee_query.iter_mut() {
                        if employee.id == target_id {
                            info!("Firing employee: {}", employee.name);
                            internal_event_sender
                                .tx_internal_events
                                .try_send(InternalEvent::SetEmployeeStatus {
                                    target_id: employee.id,
                                    status: EmploymentStatus::Fired,
                                })
                                .unwrap();

                            internal_event_sender
                                .tx_internal_events
                                .try_send(InternalEvent::DecrementReputation { amount: 1 })
                                .unwrap();

                            for org in organization_query.iter_mut() {
                                match org.vp {
                                    None => {}
                                    Some(org_vp_id) => {
                                        if org_vp_id == employee.id {
                                            internal_event_sender
                                                .tx_internal_events
                                                .try_send(InternalEvent::RemoveOrgVp {
                                                    target_id: org.id,
                                                })
                                                .unwrap();
                                        }
                                    }
                                }
                            }
                            break;
                        }
                    }
                }

                PlayerAction::GiveRaise(target_id, raise_amount) => {
                    for (employee, _, _, _) in employee_query.iter_mut() {
                        if employee.id == target_id {
                            internal_event_sender
                                .tx_internal_events
                                .try_send(InternalEvent::IncrementEmployeeSatisfaction {
                                    target_id: employee.id,
                                    amount: 1,
                                })
                                .unwrap();

                            internal_event_sender
                                .tx_internal_events
                                .try_send(InternalEvent::IncrementSalary {
                                    target_id: employee.id,
                                    amount: raise_amount,
                                })
                                .unwrap();
                        }
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

                PlayerAction::PromoteToVp { target_id, employee_id } => {
                    internal_event_sender
                        .tx_internal_events
                        .try_send(InternalEvent::SetOrgVp { target_id, employee_id })
                        .unwrap();
                }
            },
        }

        let total_productivity: u32 = employee_query
            .iter()
            .filter(|(emp, _, _, _)| emp.employment_status == EmploymentStatus::Active)
            .map(|(_, _, sat, _)| sat.0 as u32)
            .sum();

        let total_expenses: u32 = employee_query
            .iter()
            .filter(|(emp, _, _, _)| emp.employment_status == EmploymentStatus::Active)
            .map(|(_, sal, _, _)| sal.0 as u32 / 52)
            .sum();

        internal_event_sender
            .tx_internal_events
            .try_send(InternalEvent::IncrementMoney {
                amount: total_productivity * 15,
            })
            .unwrap();

        internal_event_sender
            .tx_internal_events
            .try_send(InternalEvent::DecrementMoney {
                amount: total_expenses,
            })
            .unwrap();

        internal_event_sender
            .tx_internal_events
            .try_send(InternalEvent::AdvanceWeek)
            .unwrap();
    }
}
