use crate::NeedsWorldBroadcast;
use crate::systems::ServerEventSender;
use bevy::prelude::{Query, Res, ResMut};
use shared::{
    Child, ChildSnapshot, Employee, EmployeeSnapshot, GameStateSnapshot, InternalEntity, Level,
    Money, Organization, OrganizationMember, OrganizationSnapshot, Pet, PetSnapshot, Player,
    Reputation, Salary, Satisfaction, ServerEvent, Week,
};
use std::collections::HashMap;
use uuid::Uuid;

pub fn process_broadcast_world_state(
    mut needs_broadcast: ResMut<NeedsWorldBroadcast>,
    query_money: Query<&Money>,
    query_rep: Query<&Reputation>,
    query_player: Query<(&Player, &Money, &Reputation, &Week, Option<&InternalEntity>)>,
    query_organizations: Query<&Organization>,
    query_org_members: Query<(
        &OrganizationMember,
        &Employee,
        &Salary,
        &Satisfaction,
        &Level,
    )>,
    query_pet: Query<&Pet>,
    query_child: Query<&Child>,
    server_event_sender: Res<ServerEventSender>,
) {
    // Only send if there's a connected player and we're due to broadcast
    let (_, _, _, week, internal_entity) = query_player.single();
    if internal_entity.is_none() || !needs_broadcast.0 {
        return;
    }
    needs_broadcast.0 = false;

    let money = query_money.single().0;
    let reputation = query_rep.single().0;

    let mut org_map: HashMap<Uuid, Vec<EmployeeSnapshot>> = HashMap::new();

    for (org_member, emp, sal, sat, lvl) in query_org_members.iter() {
        let emp_snapshot = EmployeeSnapshot {
            id: emp.id,
            name: emp.name.clone(),
            satisfaction: sat.0,
            employment_status: emp.employment_status.clone(),
            salary: sal.0,
            role: emp.role.clone(),
            level: lvl.0,
            organization_id: Some(org_member.organization_id.clone()),
            org_role: Some(org_member.role.clone()),
            children_ids: query_child
                .iter()
                .filter(|child| child.parent_id == emp.id)
                .map(|child| child.id)
                .collect(),
            pet_ids: query_pet
                .iter()
                .filter(|pet| pet.owner_id == emp.id)
                .map(|pet| pet.id)
                .collect(),
        };

        org_map
            .entry(org_member.organization_id)
            .or_default()
            .push(emp_snapshot);
    }

    let organizations = query_organizations
        .iter()
        .map(|org| OrganizationSnapshot {
            id: org.id,
            name: org.name.clone(),
            vp: org.vp,
            employees: org_map.remove(&org.id).unwrap_or_default(),
        })
        .collect::<Vec<_>>();

    let pets = query_pet
        .iter()
        .map(|pet| PetSnapshot {
            id: pet.id,
            name: pet.name.clone(),
            pet_type: pet.pet_type.clone(),
        })
        .collect::<Vec<_>>();

    let children = query_child
        .iter()
        .map(|child| ChildSnapshot {
            id: child.id,
            name: child.name.clone(),
        })
        .collect::<Vec<_>>();

    let snapshot = GameStateSnapshot {
        week: week.0,
        money,
        reputation,
        organizations,
        pets,
        children,
    };

    let _ = server_event_sender
        .tx_server_events
        .try_send(ServerEvent::FullState(snapshot));
}
