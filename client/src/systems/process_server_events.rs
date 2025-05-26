use crate::systems::ServerEventsReceiver;
use bevy::prelude::{Res, ResMut};
use renderer_api::{ClientGameState, ClientHistoryState};
use shared::{EntityType, HelloState, ServerEvent};
use std::cmp::Reverse;
use std::collections::HashMap;
use tracing::{debug, error, info};
use uuid::Uuid;

pub fn process_server_events(
    server_events_receiver: Res<ServerEventsReceiver>,
    mut game_state_snapshot: ResMut<ClientGameState>,
    mut history_state_snapshot: ResMut<ClientHistoryState>,
) {
    let received = server_events_receiver.rx_server_events.try_recv();
    if received.is_err() {
        debug!(
            "server_events_receiver.rx_server_events.try_recv(), {:?}",
            received
        );
        return;
    }

    match received.unwrap() {
        ServerEvent::None => info!("Client has an empty server event"),
        ServerEvent::Hello(HelloState::Accepted) => {
            info!("Client connection accepted");
        }
        ServerEvent::Hello(HelloState::Rejected { reason }) => {
            error!("Client connection rejected: {}", reason)
        }
        ServerEvent::FullState(rx_game_state) => {
            game_state_snapshot.week = rx_game_state.week;
            game_state_snapshot.players = rx_game_state.players;
            game_state_snapshot.companies = rx_game_state.companies;
            game_state_snapshot.organizations = rx_game_state.organizations;
            game_state_snapshot.entities = rx_game_state.entities;

            let mut ordered_organizations_of_company: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
            let mut ordered_employees_of_organization: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
            let mut ordered_employees_of_company: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
            let mut ordered_unemployed_entities: Vec<Uuid> = vec![];
            let mut ordered_pets_of_entity: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
            let mut ordered_children_of_entity: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

            for entity in game_state_snapshot.entities.values() {
                // Employment relationships
                if let Some(employment) = &entity.employment {
                    // By organization
                    ordered_employees_of_organization
                        .entry(employment.organization_id)
                        .or_default()
                        .push(entity.id);

                    // By company (via organization -> company_relation)
                    if let Some(org) = game_state_snapshot
                        .organizations
                        .get(&employment.organization_id)
                    {
                        ordered_employees_of_company
                            .entry(org.company_relation.entity_id)
                            .or_default()
                            .push(entity.id);
                    }
                } else {
                    // Unemployed
                    ordered_unemployed_entities.push(entity.id);
                }

                // Ownership relationships (for pets and children)
                if let Some(owner) = &entity.owner {
                    match entity.entity_type {
                        EntityType::Human(_) => {
                            ordered_children_of_entity
                                .entry(owner.entity_id)
                                .or_default()
                                .push(entity.id);
                        }
                        _ => {
                            ordered_pets_of_entity
                                .entry(owner.entity_id)
                                .or_default()
                                .push(entity.id);
                        }
                    }
                }
            }

            for (_organization_id, organization) in game_state_snapshot.organizations.clone() {
                ordered_organizations_of_company
                    .entry(organization.company_relation.entity_id)
                    .or_default()
                    .push(organization.id);
            }

            for list in ordered_organizations_of_company.values_mut() {
                list.sort_by_key(|e| {
                    game_state_snapshot
                        .organizations
                        .get(e)
                        .unwrap()
                        .name
                        .clone()
                });
            }
            for list in ordered_employees_of_organization.values_mut() {
                list.sort_by_key(|e| {
                    Reverse(
                        game_state_snapshot
                            .entities
                            .get(e)
                            .unwrap()
                            .clone()
                            .employment
                            .unwrap()
                            .level,
                    )
                });
            }
            for list in ordered_employees_of_company.values_mut() {
                list.sort_by_key(|e| {
                    Reverse(
                        game_state_snapshot
                            .entities
                            .get(e)
                            .unwrap()
                            .clone()
                            .employment
                            .unwrap()
                            .level,
                    )
                });
            }
            for list in ordered_pets_of_entity.values_mut() {
                list.sort_by_key(|e| game_state_snapshot.entities.get(e).unwrap().name.clone());
            }
            for list in ordered_children_of_entity.values_mut() {
                list.sort_by_key(|e| game_state_snapshot.entities.get(e).unwrap().name.clone());
            }

            ordered_unemployed_entities
                .sort_by_key(|e| game_state_snapshot.entities.get(e).unwrap().name.clone());

            game_state_snapshot.ordered_organizations_of_company = ordered_organizations_of_company;
            game_state_snapshot.ordered_employees_of_organization =
                ordered_employees_of_organization;
            game_state_snapshot.ordered_employees_of_company = ordered_employees_of_company;
            game_state_snapshot.ordered_unemployed_entities = ordered_unemployed_entities;
            game_state_snapshot.ordered_pets_of_entity = ordered_pets_of_entity;
            game_state_snapshot.ordered_children_of_entity = ordered_children_of_entity;
        }
        ServerEvent::HistoryState(rx_history) => {
            history_state_snapshot.history_state = rx_history.clone();
            history_state_snapshot.player_order = rx_history.players.keys().cloned().collect();
            history_state_snapshot.organization_order =
                rx_history.organizations.keys().cloned().collect();
            history_state_snapshot.company_order = rx_history.companies.keys().cloned().collect();
        }
    }
}
