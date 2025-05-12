use crate::NeedsWorldBroadcast;
use crate::systems::FanOutClientCommandReceiver;
use bevy::prelude::{Mut, Query, Res, ResMut};
use shared::{
    ClientCommand, Employee, EmploymentStatus, Money, PlayerAction, Reputation, Salary,
    Satisfaction,
};
use tracing::info;

pub fn process_client_commands(
    channel: Res<FanOutClientCommandReceiver>,
    mut needs_broadcast: ResMut<NeedsWorldBroadcast>,
    mut query: Query<(&mut Employee, &mut Salary, &mut Satisfaction)>,
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
                    let mut reputation = reputation.single_mut();
                    for (mut employee, _salary, _) in query.iter_mut() {
                        if employee.id == target_id {
                            process_fire_employee(&mut employee, &mut reputation);
                            break;
                        }
                    }
                }

                PlayerAction::GiveRaise(target_id, raise_amount) => {
                    for (employee, mut salary, mut satisfaction) in query.iter_mut() {
                        if employee.id == target_id {
                            process_give_raise(&mut satisfaction, &mut salary, raise_amount);
                        }
                    }
                }

                PlayerAction::LaunchPRCampaign => {
                    for mut rep in reputation.iter_mut() {
                        rep.0 += 10;
                    }

                    for mut m in money.iter_mut() {
                        m.0 -= 10000;
                    }
                }

                PlayerAction::DoNothing => {
                    info!("Player did nothing this turn.");
                }
            },
        }

        needs_broadcast.0 = true;
    }
}

fn process_give_raise(
    satisfaction: &mut Mut<Satisfaction>,
    salary: &mut Mut<Salary>,
    raise_amount: i32,
) {
    satisfaction.0 += 1;
    salary.0 += raise_amount;
}

fn process_fire_employee(employee: &mut Mut<Employee>, reputation: &mut Mut<Reputation>) {
    info!("Firing employee: {}", employee.name);
    employee.employment_status = EmploymentStatus::Fired;
    reputation.0 = reputation.0 - 1;
}
