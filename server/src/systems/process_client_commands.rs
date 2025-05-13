use crate::NeedsWorldBroadcast;
use crate::systems::FanOutClientCommandReceiver;
use bevy::prelude::{Mut, Query, Res, ResMut};
use shared::{
    ClientCommand, Employee, EmploymentStatus, Money, PlayerAction, Productivity,
    Reputation, Salary, Satisfaction, Week,
};
use tracing::info;

pub fn process_client_commands(
    channel: Res<FanOutClientCommandReceiver>,
    mut needs_broadcast: ResMut<NeedsWorldBroadcast>,
    mut employees: Query<(
        &mut Employee,
        &mut Salary,
        &mut Satisfaction,
        &mut Productivity,
    )>,
    mut week: Query<&mut Week>,
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
                    for (mut employee, _salary, _, _) in employees.iter_mut() {
                        if employee.id == target_id {
                            process_fire_employee(&mut employee, &mut reputation);
                            break;
                        }
                    }
                    advance_a_week(&mut employees, &mut week, &mut money);
                }

                PlayerAction::GiveRaise(target_id, raise_amount) => {
                    for (employee, mut salary, mut satisfaction, _) in employees.iter_mut() {
                        if employee.id == target_id {
                            process_give_raise(&mut satisfaction, &mut salary, raise_amount);
                        }
                    }
                    advance_a_week(&mut employees, &mut week, &mut money);
                }

                PlayerAction::LaunchPRCampaign => {
                    for mut rep in reputation.iter_mut() {
                        rep.0 += 10;
                    }

                    for mut m in money.iter_mut() {
                        m.0 -= 10000;
                    }
                    advance_a_week(&mut employees, &mut week, &mut money);
                }

                PlayerAction::DoNothing => {
                    info!("Player did nothing this turn.");
                    advance_a_week(&mut employees, &mut week, &mut money);
                }
            },
        }

        needs_broadcast.0 = true;
    }
}

fn advance_a_week(
    employees: &mut Query<(
        &mut Employee,
        &mut Salary,
        &mut Satisfaction,
        &mut Productivity,
    )>,
    week: &mut Query<&mut Week>,
    money: &mut Query<&mut Money>,
) {
    let mut week = week.single_mut();
    week.0 += 1;

    let mut money = money.single_mut();
    let total_expenses: i32 = employees
        .iter()
        .filter(|(emp, _, _, _)| emp.employment_status == EmploymentStatus::Active)
        .map(|(_, sal, _, _)| (sal.0 / 52))
        .sum();

    employees
        .iter_mut()
        .filter(|(emp, _, _, _)| emp.employment_status == EmploymentStatus::Active)
        .for_each(|(_, _, mut sat, _)| sat.0 = (sat.0 - 1).max(0));

    let total_productivity: i32 = employees
        .iter()
        .filter(|(emp, _, _, _)| emp.employment_status == EmploymentStatus::Active)
        .map(|(_, _, sat, _)| sat.0)
        .sum();

    money.0 += total_productivity * 50;
    money.0 -= total_expenses;
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
