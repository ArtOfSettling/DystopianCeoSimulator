use crate::NeedsWorldBroadcast;
use crate::systems::FanOutEventReceiver;
use bevy::prelude::{Entity, Query, Res, ResMut};
use shared::components::{Employee, InternalEntity};
use shared::{InternalEvent, Money, Organization, Player, Reputation, Salary, Satisfaction, Week};
use tracing::info;

pub fn process_internal_events(
    mut needs_broadcast: ResMut<NeedsWorldBroadcast>,
    channel: Res<FanOutEventReceiver>,
    mut player_query: Query<(
        Entity,
        &Player,
        &mut Money,
        &mut Reputation,
        &mut Week,
        Option<&InternalEntity>,
    )>,
    mut employee_query: Query<(&mut Employee, &mut Salary, &mut Satisfaction)>,
    mut organizations: Query<&mut Organization>,
) {
    while let Ok(internal_event) = channel.rx_fan_out_events.try_recv() {
        info!(
            "Server has internal event for processing {:?}",
            internal_event
        );
        needs_broadcast.0 = true;

        match internal_event {
            InternalEvent::SetEmployeeStatus { target_id, status } => {
                for (mut emp, _, _) in employee_query.iter_mut() {
                    if emp.id == target_id {
                        emp.employment_status = status;
                        break;
                    }
                }
            }

            InternalEvent::DecrementReputation { amount } => {
                for (_, _, _, mut rep, _, _) in player_query.iter_mut() {
                    rep.0 -= amount as i32;
                }
            }

            InternalEvent::DecrementMoney { amount } => {
                for (_, _, mut money, _, _, _) in player_query.iter_mut() {
                    money.0 -= amount as i32;
                }
            }

            InternalEvent::IncrementMoney { amount } => {
                let (_, _, mut money, _, _, _) = player_query.single_mut();
                money.0 += amount as i32;
            }

            InternalEvent::IncrementEmployeeSatisfaction { target_id, amount } => {
                for (emp, _, mut sat) in employee_query.iter_mut() {
                    if emp.id == target_id {
                        sat.0 += amount as i32;
                        break;
                    }
                }
            }

            InternalEvent::IncrementSalary { target_id, amount } => {
                for (emp, mut salary, _) in employee_query.iter_mut() {
                    if emp.id == target_id {
                        salary.0 += amount as i32;
                        break;
                    }
                }
            }

            InternalEvent::IncrementReputation { amount } => {
                for (_, _, _, mut rep, _, _) in player_query.iter_mut() {
                    rep.0 += amount as i32;
                }
            }

            InternalEvent::RemoveOrgVp { target_id } => {
                for mut org in organizations.iter_mut() {
                    if org.id == target_id {
                        org.vp = None;
                    }
                }
            }

            InternalEvent::AdvanceWeek => {
                let (_, _, _, _, mut week, _) = player_query.single_mut();
                week.0 += 1;
            }
        }
    }
}
