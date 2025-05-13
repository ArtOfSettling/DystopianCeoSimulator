use crate::NeedsAdvanceAWeek;
use bevy::prelude::{Query, ResMut};
use shared::{Employee, EmploymentStatus, Money, Productivity, Salary, Satisfaction, Week};

pub fn process_advance_week(
    mut needs_advance_a_week: ResMut<NeedsAdvanceAWeek>,
    mut employees: Query<(
        &mut Employee,
        &mut Salary,
        &mut Satisfaction,
        &mut Productivity,
    )>,
    mut week: Query<&mut Week>,
    mut money: Query<&mut Money>,
) {
    if !needs_advance_a_week.0 {
        return;
    }

    needs_advance_a_week.0 = false;

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

    money.0 += total_productivity * 15;
    money.0 -= total_expenses;
}
