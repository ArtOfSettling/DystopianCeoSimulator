use crate::systems::FanOutClientCommandReceiver;
use crate::{InternalEventSender, NeedsStateUpdate};
use bevy::prelude::{Query, Res, ResMut, With, Without};
use shared::{
    ClientCommand, Employed, InternalEntity, InternalEvent, Name, PlayerAction, Salary,
    Satisfaction,
};
use tracing::info;

pub fn process_client_commands(
    channel: Res<FanOutClientCommandReceiver>,
    mut needs_state_update: ResMut<NeedsStateUpdate>,
    internal_event_sender: Res<InternalEventSender>,
    employee_query: Query<(&InternalEntity, &Name, &Salary, &Satisfaction), With<Employed>>,
    entity_query: Query<(&InternalEntity, &Name), Without<Employed>>,
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
                    for (internal_entity, name, _, _) in employee_query.iter() {
                        if internal_entity.id == employee_id {
                            info!("Firing employee: {}", name.0);
                            internal_event_sender
                                .tx_internal_events
                                .try_send(InternalEvent::RemoveEmployedStatus {
                                    employee_id: internal_entity.id,
                                })
                                .unwrap();
                            break;
                        }
                    }
                }

                PlayerAction::HireEmployee {
                    organization_id,
                    employee_id,
                } => {
                    for (internal_entity, name) in entity_query.iter() {
                        if internal_entity.id == employee_id {
                            info!("Hiring employee: {}", name.0);
                            internal_event_sender
                                .tx_internal_events
                                .try_send(InternalEvent::AddEmployedStatus {
                                    organization_id,
                                    employee_id,
                                })
                                .unwrap();
                            break;
                        }
                    }
                }

                PlayerAction::GiveRaise {
                    employee_id,
                    amount,
                } => {
                    for (internal_entity, _, _, _) in employee_query.iter() {
                        if internal_entity.id == employee_id {
                            internal_event_sender
                                .tx_internal_events
                                .try_send(InternalEvent::IncrementEmployeeSatisfaction {
                                    employee_id,
                                    amount: 1,
                                })
                                .unwrap();

                            internal_event_sender
                                .tx_internal_events
                                .try_send(InternalEvent::IncrementSalary {
                                    employee_id,
                                    amount,
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

                PlayerAction::PromoteToVp {
                    organization_id,
                    employee_id,
                } => {
                    internal_event_sender
                        .tx_internal_events
                        .try_send(InternalEvent::SetOrgVp {
                            organization_id,
                            employee_id,
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
                        organization_budget,
                    })
                    .unwrap(),
            },
        }

        let total_productivity: u32 = employee_query
            .iter()
            .map(|(_, _, _, sat)| sat.0 as u32)
            .sum();

        let total_expenses: u32 = employee_query
            .iter()
            .map(|(_, _, sal, _)| sal.0 as u32 / 52)
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
