use crate::deterministic_randomization::{
    PetKind, build_all_company_name_pool, build_all_human_name_pools, build_all_org_name_pools,
    build_all_pet_name_pools, generate_human_type_for_organization_role,
    generate_organization_chart, generate_organization_types_for_company,
    generate_pet_type_for_rank, generate_unique_pet_name,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};
use shared::{
    Budget, Company, CompanyRelation, CompanyType, Employment, Entity, EntityFlag, EntityType,
    Financials, GameState, Organization, OrganizationRole, Origin, Owner, Perception, Player,
};
use std::collections::{HashMap, VecDeque};
use tracing::info;
use uuid::Uuid;

pub fn create_empty_world_state() -> GameState {
    let mut new_game_state = generate_game_state_deterministic(12345, 0, 7);

    info!("spawning player");
    new_game_state.players.push(Player {
        id: None,
        financials: Financials {
            actual_cash: 1_000_000,
            this_weeks_income: 0,
            this_weeks_expenses: 0,
            this_weeks_net_profit: 0,
        },
        perception: Perception {
            public_opinion: 0,
            reputation: 0,
        },
    });

    new_game_state
}

pub fn generate_game_state_deterministic(seed: u64, week: u16, org_count: usize) -> GameState {
    let mut rng = StdRng::seed_from_u64(seed);

    // Hard Coded for now, randomize later.
    let company_type = CompanyType::ECommerce;
    let organization_types =
        generate_organization_types_for_company(&mut rng, &company_type, org_count);

    let mut company_name_pool = build_all_company_name_pool(&mut rng, &company_type);
    let mut human_name_pool = build_all_human_name_pools(&mut rng);
    let mut org_name_pool = build_all_org_name_pools(&mut rng);
    let mut pet_name_pool = build_all_pet_name_pools(&mut rng);

    let mut uuid_counter = 0;

    let mut game_state = GameState {
        week,
        players: vec![],
        companies: HashMap::new(),
        organizations: HashMap::new(),
        entities: HashMap::new(),
    };

    let company_id = deterministic_uuid(seed, uuid_counter);
    let company = Company {
        id: company_id,
        name: company_name_pool.pop_back().unwrap(),
        company_type: company_type.clone(),
        perception: Perception::default(),
        financials: Financials::default(),
    };
    game_state.companies.insert(company_id, company);

    uuid_counter += 1;
    for organization_type in organization_types {
        let organization_chart = generate_organization_chart(organization_type, &mut rng, 9, 24);

        let org_id = deterministic_uuid(seed, uuid_counter);
        uuid_counter += 1;
        let org_name = org_name_pool
            .get_mut(&organization_type)
            .unwrap()
            .pop_back()
            .unwrap();

        let mut employee_ids = vec![];
        let vp_id = deterministic_uuid(seed, uuid_counter);
        uuid_counter += 1;
        employee_ids.push(vp_id);

        let organization_role = OrganizationRole::VP;
        let human_type =
            generate_human_type_for_organization_role(&organization_role, &mut rng).unwrap();
        let human_name = human_name_pool
            .get_mut(&(human_type, organization_role))
            .unwrap()
            .pop_back()
            .unwrap();

        let vp = Entity {
            id: vp_id,
            name: human_name,
            entity_type: EntityType::Human(human_type),
            employment: Some(Employment {
                organization_id: org_id,
                role: OrganizationRole::VP,
                employee_flags: vec![],
                level: rng.gen_range(16..=20),
                salary: rng.gen_range(950u16..=2_000u16),
                satisfaction: rng.gen_range(6_700u16..=9_600u16),
                productivity: rng.gen_range(7_200u16..=9_100u16),
            }),
            owner: None,
            origin: Origin {
                week_of_birth: -rng.gen_range(200..=400),
            },
            flags: vec![],
        };
        game_state.entities.insert(vp_id, vp);

        for org_chart_employee in organization_chart.employees {
            let id = deterministic_uuid(seed, uuid_counter);
            uuid_counter += 1;

            let organization_role = org_chart_employee.role;
            let human_name = human_name_pool
                .get_mut(&(org_chart_employee.human_type, organization_role))
                .unwrap()
                .pop_back()
                .unwrap();

            let level = match org_chart_employee.rank {
                0 => rng.gen_range(12..=15),
                1 => rng.gen_range(8..=11),
                2 => rng.gen_range(6..=8),
                3 => rng.gen_range(1..=5),
                _ => 1,
            };

            let salary = ((level as f32 * 0.8) + (rng.gen_range(0.7..=0.9) * level as f32)) as u16;
            let satisfaction = rng.gen_range(50..=100);
            let productivity = rng.gen_range(60..=100);

            let mut flags = Vec::new();

            if rng.gen_bool(0.02) {
                flags.push(EntityFlag::Hoarder);
            }

            let employee = Entity {
                id,
                name: human_name.clone(),
                entity_type: EntityType::Human(org_chart_employee.human_type),
                employment: Some(Employment {
                    organization_id: org_id,
                    role: org_chart_employee.role,
                    employee_flags: vec![],
                    level,
                    salary,
                    satisfaction,
                    productivity,
                }),
                owner: None,
                origin: Origin {
                    week_of_birth: -rng.gen_range(700..=2_400),
                },
                flags,
            };
            game_state.entities.insert(id, employee.clone());
            employee_ids.push(id);

            let is_hoarder = employee.flags.contains(&EntityFlag::Hoarder);
            let number_of_pets = if is_hoarder {
                rng.gen_range(5..11)
            } else if rng.gen_bool(0.1) {
                1
            } else {
                0
            };

            for _ in 0..number_of_pets {
                let pet = generate_pet_for_role(
                    org_chart_employee.rank,
                    employee.id,
                    &mut pet_name_pool,
                    &mut rng,
                    seed,
                    &mut uuid_counter,
                );
                game_state.entities.insert(pet.id, pet);
            }
        }

        // Create organization
        let organization = Organization {
            id: org_id,
            name: org_name,
            organization_type: organization_chart.organization_type,
            vp: Some(vp_id),
            company_relation: CompanyRelation {
                entity_id: company_id,
            },
            financials: Financials::default(),
            perception: Perception::default(),
            budget: Budget {
                marketing: 10,
                rnd: 10,
                training: 10,
            },
            initiatives: vec![],
        };

        game_state.organizations.insert(org_id, organization);
    }

    game_state
}

fn generate_pet_for_role(
    rank: u32,
    owner_id: Uuid,
    pet_name_pool: &mut HashMap<PetKind, VecDeque<String>>,
    rng: &mut StdRng,
    seed: u64,
    uuid_counter: &mut u64,
) -> Entity {
    let entity_type = generate_pet_type_for_rank(rank as usize, rng);
    let id = deterministic_uuid(seed, *uuid_counter);
    *uuid_counter += 1;

    let name = generate_unique_pet_name(&entity_type, pet_name_pool, rng).unwrap();

    Entity {
        id,
        name,
        entity_type,
        employment: None,
        owner: Some(Owner {
            entity_id: owner_id,
        }),
        origin: Origin {
            week_of_birth: rng.gen_range(10..100),
        },
        flags: vec![],
    }
}

fn deterministic_uuid(seed: u64, counter: u64) -> Uuid {
    let mut hasher = Sha256::new();
    hasher.update(seed.to_le_bytes());
    hasher.update(counter.to_le_bytes());
    let hash = hasher.finalize();
    let bytes: [u8; 16] = hash[0..16].try_into().unwrap();
    Uuid::from_bytes(bytes)
}
