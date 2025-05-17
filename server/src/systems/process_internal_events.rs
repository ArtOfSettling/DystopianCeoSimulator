use crate::NeedsWorldBroadcast;
use crate::systems::FanOutEventReceiver;
use bevy::ecs::system::SystemState;
use bevy::prelude::{Entity, QueryState, Res, ResMut, Without, World};
use shared::components::InternalEntity;
use shared::{
    Company, Employed, EmployeeFlags, InternalEvent, Level, Money, Name, OrgRole, Organization,
    Player, Productivity, Reputation, Salary, Satisfaction, Week,
};
use tracing::info;

#[allow(clippy::type_complexity)]
pub fn process_internal_events(
    world: &mut World,
    player_query: &mut QueryState<(
        Entity,
        &Player,
        &mut Money,
        &mut Reputation,
        &mut Week,
        Option<&InternalEntity>,
    )>,
    employee_query: &mut QueryState<(
        Entity,
        &mut InternalEntity,
        &mut Name,
        &mut Employed,
        &mut Salary,
        &mut Satisfaction,
    )>,
    entity_query: &mut QueryState<(Entity, &mut InternalEntity), Without<Employed>>,
    organizations: &mut QueryState<&mut Organization>,
    params: &mut SystemState<(
        ResMut<Company>,
        ResMut<NeedsWorldBroadcast>,
        Res<FanOutEventReceiver>,
    )>,
) {
    let (mut company, mut needs_world_broadcast, fan_out_event_receiver) = params.get_mut(world);

    if let Ok(internal_event) = fan_out_event_receiver.rx_fan_out_events.try_recv() {
        info!(
            "Server has internal event for processing {:?}",
            internal_event
        );
        needs_world_broadcast.0 = true;

        match internal_event {
            InternalEvent::RemoveEmployedStatus { employee_id } => {
                for (entity, internal_entity, _, _, _, _) in employee_query.iter_mut(world) {
                    if internal_entity.id == employee_id {
                        world.entity_mut(entity).remove::<Employed>();
                        break;
                    }
                }
            }

            InternalEvent::AddEmployedStatus {
                organization_id,
                employee_id,
            } => {
                for (entity, internal_entity) in entity_query.iter_mut(world) {
                    if internal_entity.id == employee_id {
                        world.entity_mut(entity).insert(Employed {
                            owner_id: organization_id,
                            role: OrgRole::Employee,
                        });

                        world.entity_mut(entity).insert_if_new((
                            Level(10_000),
                            Satisfaction(80),
                            Productivity(80),
                            Salary(7_000),
                            EmployeeFlags(vec![]),
                        ));

                        break;
                    }
                }
            }

            InternalEvent::DecrementReputation { amount } => {
                for (_, _, _, mut rep, _, _) in player_query.iter_mut(world) {
                    rep.0 -= amount as i32;
                }
            }

            InternalEvent::DecrementMoney { amount } => {
                for (_, _, mut money, _, _, _) in player_query.iter_mut(world) {
                    money.0 -= amount as i32;
                }
            }

            InternalEvent::IncrementMoney { amount } => {
                let (_, _, mut money, _, _, _) = player_query.single_mut(world);
                money.0 += amount as i32;
            }

            InternalEvent::IncrementEmployeeSatisfaction {
                employee_id,
                amount,
            } => {
                for (_, internal_entity, _, _, _, mut sat) in employee_query.iter_mut(world) {
                    if internal_entity.id == employee_id {
                        sat.0 += amount as i32;
                        break;
                    }
                }
            }

            InternalEvent::IncrementSalary {
                employee_id,
                amount,
            } => {
                for (_, internal_entity, _, _, mut salary, _) in employee_query.iter_mut(world) {
                    if internal_entity.id == employee_id {
                        salary.0 += amount as i32;
                        break;
                    }
                }
            }

            InternalEvent::IncrementReputation { amount } => {
                for (_, _, _, mut rep, _, _) in player_query.iter_mut(world) {
                    rep.0 += amount as i32;
                }
            }

            InternalEvent::RemoveOrgVp { organization_id } => {
                for mut org in organizations.iter_mut(world) {
                    if org.id == organization_id {
                        org.vp = None;
                    }
                }
            }

            InternalEvent::AdvanceWeek => {
                let (_, _, _, _, mut week, _) = player_query.single_mut(world);
                week.0 += 1;
            }

            InternalEvent::SetOrgVp {
                organization_id,
                employee_id,
            } => {
                // Replace the existing vp
                let mut vp_to_remove = None;
                for mut org in organizations.iter_mut(world) {
                    if org.id == organization_id {
                        vp_to_remove = org.vp;
                        org.vp = Some(employee_id);
                        break;
                    }
                }

                // update employee titles
                for (_entity, internal_entity, _name, mut employed, _salary, _satisfaction) in
                    employee_query.iter_mut(world)
                {
                    if internal_entity.id == employee_id {
                        employed.role = OrgRole::VP;
                        continue;
                    }
                    if vp_to_remove.is_none() {
                        continue;
                    }

                    if internal_entity.id == vp_to_remove.unwrap() {
                        employed.role = OrgRole::Employee;
                        continue;
                    }
                }
            }

            InternalEvent::SetOrgFinancials {
                organization_id,
                financials,
            } => {
                for mut org in organizations.iter_mut(world) {
                    if org.id == organization_id {
                        org.financials = financials.clone();
                        break;
                    }
                }
            }

            InternalEvent::SetCompanyFinancials { financials } => {
                company.financials = financials.clone();
            }
        }
    }
}
