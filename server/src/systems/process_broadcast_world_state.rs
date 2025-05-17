use crate::NeedsWorldBroadcast;
use crate::systems::ServerEventSender;
use bevy::prelude::{Query, Res, ResMut, Without};
use shared::{AnimalSnapshot, Company, Employed, EmployeeSnapshot, EntityType, GameStateSnapshot, HumanSnapshot, InternalEntity, Level, Money, Name, Organization, OrganizationSnapshot, Owner, Player, PublicOpinion, Reputation, Salary, Satisfaction, ServerEvent, Type, UnemployedSnapshot, Week, WeekOfBirth};
use std::collections::HashMap;
use uuid::Uuid;

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn process_broadcast_world_state(
    mut needs_broadcast: ResMut<NeedsWorldBroadcast>,
    company: Res<Company>,
    query_player: Query<(&Player, &Money, &Reputation, &PublicOpinion, &Week, Option<&InternalEntity>)>,
    query_organizations: Query<(&Organization, &Reputation, &PublicOpinion)>,
    employee_query: Query<(
        &InternalEntity,
        &Name,
        &Employed,
        &Salary,
        &Satisfaction,
        Option<&Owner>,
        &Level,
        &Type,
        &WeekOfBirth,
    )>,
    entity_query: Query<
        (&InternalEntity, &Name, Option<&Owner>, &Type, &WeekOfBirth),
        Without<Employed>,
    >,
    server_event_sender: Res<ServerEventSender>,
) {
    // Only send if there's a connected player and we're due to broadcast
    let (_, _, _, _, week, internal_entity) = query_player.single();
    if internal_entity.is_none() || !needs_broadcast.0 {
        return;
    }
    needs_broadcast.0 = false;

    let (_, _, reputation, public_opinion, _, _) = query_player.single();

    let mut org_map: HashMap<Uuid, Vec<EmployeeSnapshot>> = HashMap::new();

    for (internal_entity, name, employed, sal, sat, _, lvl, type_, week_of_birth) in
        employee_query.iter()
    {
        let emp_snapshot = EmployeeSnapshot {
            id: internal_entity.id,
            name: name.0.clone(),
            satisfaction: sat.0,
            salary: sal.0,
            role: employed.role.clone(),
            level: lvl.0,
            entity_type: type_.0.clone(),
            organization_id: Some(employed.owner_id),
            week_of_birth: week_of_birth.0,
            children_ids: entity_query
                .iter()
                .filter(|(_, _, owner, _, _)| owner.is_some())
                .filter(|(_, _, owner, _, _)| {
                    owner.unwrap().owner_id.unwrap() == internal_entity.id
                })
                .filter(|(_, _, _, type_, _)| type_.0 == EntityType::Human)
                .map(|(internal_entity, _, _, _, _)| internal_entity.id)
                .collect(),
            pet_ids: entity_query
                .iter()
                .filter(|(_, _, owner, _, _)| owner.is_some())
                .filter(|(_, _, owner, _, _)| {
                    owner.unwrap().owner_id.unwrap() == internal_entity.id
                })
                .filter(|(_, _, _, type_, _)| type_.0 != EntityType::Human)
                .map(|(internal_entity, _, _, _, _)| internal_entity.id)
                .collect(),
        };

        org_map
            .entry(employed.owner_id)
            .or_default()
            .push(emp_snapshot);
    }

    let organizations = query_organizations
        .iter()
        .map(|(org, reputation, public_opinion)| OrganizationSnapshot {
            id: org.id,
            name: org.name.clone(),
            vp: org.vp,
            employees: org_map.remove(&org.id).unwrap_or_default(),
            financials: org.financials.clone(),
            reputation: reputation.0,
            public_opinion: public_opinion.0,
        })
        .collect::<Vec<_>>();

    let pets = entity_query
        .iter()
        .filter(|(_, _, _, type_, _)| type_.0 != EntityType::Human)
        .map(
            |(internal_entity, name, _owner, type_, week_of_birth)| AnimalSnapshot {
                id: internal_entity.id,
                name: name.0.clone(),
                entity_type: type_.0.clone(),
                week_of_birth: week_of_birth.0,
            },
        )
        .collect::<Vec<_>>();

    let humans = entity_query
        .iter()
        .filter(|(_, _, _, type_, _)| type_.0 == EntityType::Human)
        .map(
            |(internal_entity, name, _owner, _type_, week_of_birth)| HumanSnapshot {
                id: internal_entity.id,
                name: name.0.clone(),
                week_of_birth: week_of_birth.0,
            },
        )
        .collect::<Vec<_>>();

    let unemployed = entity_query
        .iter()
        .map(
            |(internal_entity, name, _, type_, week_of_birth)| match type_.0 {
                EntityType::Human => UnemployedSnapshot::UnemployedHumanSnapshot(HumanSnapshot {
                    id: internal_entity.id,
                    name: name.0.clone(),
                    week_of_birth: week_of_birth.0,
                }),
                _ => UnemployedSnapshot::UnemployedAnimalSnapshot(AnimalSnapshot {
                    id: internal_entity.id,
                    name: name.0.clone(),
                    entity_type: type_.0.clone(),
                    week_of_birth: week_of_birth.0,
                }),
            },
        )
        .collect::<Vec<_>>();

    let snapshot = GameStateSnapshot {
        week: week.0,
        financials: company.financials.clone(),
        reputation: reputation.0,
        public_opinion: public_opinion.0,
        company_reputation: company.reputation,
        company_public_opinion: company.public_opinion,
        organizations,
        humans,
        pets,
        unemployed,
    };

    let _ = server_event_sender
        .tx_server_events
        .try_send(ServerEvent::FullState(snapshot));
}
