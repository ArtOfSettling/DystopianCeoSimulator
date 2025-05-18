use crate::NeedsWorldBroadcast;
use crate::systems::ServerEventSender;
use bevy::prelude::{Query, Res, ResMut, Without};
use shared::{
    AnimalSnapshot, Company, Employed, EmployeeSnapshot, EntityType, GameStateSnapshot,
    HistoricalData, HistoryStateSnapshot, HumanSnapshot, InternalEntity, Level, Money, Name,
    OrgHistoryPoint, OrgHistorySnapshot, Organization, OrganizationHistoryEntry,
    OrganizationSnapshot, Owner, Player, PublicOpinion, Reputation, Salary, Satisfaction,
    ServerEvent, Type, UnemployedSnapshot, Week, WeekOfBirth,
};
use std::collections::HashMap;
use uuid::Uuid;

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn process_broadcast_world_state(
    mut needs_broadcast: ResMut<NeedsWorldBroadcast>,
    mut historical_data: ResMut<HistoricalData>,
    company: Res<Company>,
    query_player: Query<(
        &Player,
        &Money,
        &Reputation,
        &PublicOpinion,
        &Week,
        Option<&InternalEntity>,
    )>,
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
    let (_, _, reputation, public_opinion, week, internal_entity) = query_player.single();
    if internal_entity.is_none() || !needs_broadcast.0 {
        return;
    }
    needs_broadcast.0 = false;

    let mut children_map: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    let mut pets_map: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

    for (internal_entity, _name, owner, type_, _birth) in entity_query.iter() {
        if let Some(owner) = owner {
            let target_map = if type_.0 == EntityType::Human {
                &mut children_map
            } else {
                &mut pets_map
            };
            target_map
                .entry(owner.owner_id.unwrap())
                .or_default()
                .push(internal_entity.id);
        }
    }

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
            children_ids: children_map.remove(&internal_entity.id).unwrap_or_default(),
            pet_ids: pets_map.remove(&internal_entity.id).unwrap_or_default(),
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
            budget: org.budget.clone(),
            employees: org_map.remove(&org.id).unwrap_or_default(),
            initiatives: org.initiatives.clone(),
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

    for org in &organizations {
        let net_profit = org.financials.this_weeks_income - org.financials.this_weeks_expenses;

        historical_data
            .org_history
            .entry(org.id)
            .or_default()
            .push(OrganizationHistoryEntry {
                week: week.0 as i32,
                cash: org.financials.actual_cash,
                income: org.financials.this_weeks_income,
                expenses: org.financials.this_weeks_expenses,
                net_profit,
                public_opinion: org.public_opinion,
                reputation: org.reputation,
                avg_employee_satisfaction: if org.employees.is_empty() {
                    0
                } else {
                    org.employees.iter().map(|e| e.satisfaction).sum::<i32>()
                        / org.employees.len() as i32
                },
                budgets: org.budget.clone(),
            });
    }

    const HISTORY_WINDOW: i32 = 40;

    let history_snapshot = HistoryStateSnapshot {
        week: week.0 as i32,
        organizations: organizations
            .iter()
            .map(|org| {
                let history = historical_data
                    .org_history
                    .get(&org.id)
                    .map(|entries| {
                        let total = entries.len();
                        let start = total.saturating_sub(HISTORY_WINDOW as usize);
                        &entries[start..]
                    })
                    .unwrap_or(&[]);

                OrgHistorySnapshot {
                    org_id: org.id,
                    name: org.name.clone(),
                    recent_history: history
                        .iter()
                        .map(|entry| OrgHistoryPoint {
                            week: entry.week,
                            net_profit: entry.net_profit,
                            cash: entry.cash,
                            public_opinion: entry.public_opinion,
                            reputation: entry.reputation,
                            avg_employee_satisfaction: entry.avg_employee_satisfaction,
                        })
                        .collect(),
                }
            })
            .collect(),
    };

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

    let _ = server_event_sender
        .tx_server_events
        .try_send(ServerEvent::HistoryState(history_snapshot));
}
