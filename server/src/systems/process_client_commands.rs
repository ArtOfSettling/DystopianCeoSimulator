use crate::systems::FanOutClientCommandReceiver;
use bevy::prelude::{Commands, Entity, Query, Res};
use shared::{ClientCommand, Employee, Money, PlayerAction, Reputation, Satisfaction};
use tracing::info;

pub fn process_client_commands(
    mut commands: Commands,
    channel: Res<FanOutClientCommandReceiver>,
    mut query: Query<(Entity, &mut Employee, &mut Satisfaction)>,
    mut money: Query<&mut Money>,
    mut reputation: Query<&mut Reputation>,
) {
    while let Ok((_, client_command)) = channel.rx_fan_out_client_commands.try_recv() {
        info!(
            "Server has clients command for processing {:?}",
            client_command
        );
        match client_command {
            ClientCommand::PlayerAction(player_action) => match player_action {
                PlayerAction::FireEmployee(target_id) => {
                    for (entity, employee, _) in query.iter_mut() {
                        if employee.id == target_id {
                            info!("Firing employee: {}", employee.name);
                            commands.entity(entity).despawn();

                            for mut rep in reputation.iter_mut() {
                                rep.0 -= 5;
                            }

                            for mut m in money.iter_mut() {
                                m.0 += 5000.0; // saved severance?
                            }

                            break;
                        }
                    }
                }

                PlayerAction::GiveRaise(target_id, raise_amount) => {
                    for (_, employee, mut satisfaction) in query.iter_mut() {
                        if employee.id == target_id {
                            satisfaction.0 += (raise_amount / 10_000.0).clamp(0.0, 0.1);
                        }
                    }

                    for mut m in money.iter_mut() {
                        m.0 -= raise_amount;
                    }
                }

                PlayerAction::LaunchPRCampaign => {
                    for mut rep in reputation.iter_mut() {
                        rep.0 += 10;
                    }

                    for mut m in money.iter_mut() {
                        m.0 -= 10000.0;
                    }
                }

                PlayerAction::DoNothing => {
                    info!("Player did nothing this turn.");
                }
            },
        }
    }
}
