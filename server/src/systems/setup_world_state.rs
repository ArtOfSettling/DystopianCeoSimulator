use bevy::prelude::ResMut;
use shared::{Budget, CatBreed, Company, CompanyRelation, DogBreed, EmployeeFlag, Employment, Entity, EntityType, Financials, FishBreed, GameState, HorseBreed, Initiative, LizardBreed, Organization, OrganizationRole, Origin, Owner, Perception, Player, ServerGameState};
use uuid::Uuid;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use tracing::info;

pub fn setup_world_state(mut server_game_state: ResMut<ServerGameState>) {
    server_game_state.game_state = generate_game_state_deterministic(12345, 0, 7);

    info!("spawning player");
    server_game_state.game_state.players.push(Player {
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
}

pub fn generate_game_state_deterministic(seed: u64, week: u16, org_count: usize) -> GameState {
    let mut rng = StdRng::seed_from_u64(seed);

    let org_names = vec![
        "Quantum Koalas", "Turbo Sloths", "Pixel Platypus", "Infinite Ferrets",
        "Banana Conglomerate", "Lunar Lobsters", "Waffle Syndicate",
    ];

    let human_names = vec![
        "Alex", "Jamie", "Casey", "Taylor", "Riley", "Jordan", "Sam", "Morgan", "Drew", "Reese",
    ];

    let mut game_state = GameState {
        week: week as u16,
        players: vec![],
        companies: HashMap::new(),
        organizations: HashMap::new(),
        entities: HashMap::new(),
    };

    let company_id = Uuid::new_v4();
    for _ in 0..org_count {
        let company = Company {
            id: company_id,
            perception: Perception::default(),
            financials: Financials::default(),
        };
        game_state.companies.insert(company_id, company);

        let org_id = Uuid::new_v4();
        let org_name = org_names.choose(&mut rng).unwrap().to_string();
        let mut employee_ids = vec![];

        // VP
        let vp_id = Uuid::new_v4();
        employee_ids.push(vp_id);
        let vp_name = format!(
            "{} {}",
            human_names.choose(&mut rng).unwrap(),
            human_names.choose(&mut rng).unwrap()
        );
        let vp = Entity {
            id: vp_id,
            name: vp_name,
            entity_type: EntityType::Human,
            employment: Some(Employment {
                organization_id: org_id,
                role: OrganizationRole::VP,
                employee_flags: vec![],
                level: 5,
                salary: 200,
                satisfaction: 90,
                productivity: 100,
            }),
            owner: None,
            origin: Origin { week_of_birth: 1200 },
        };
        game_state.entities.insert(vp_id, vp);

        // Add Manager + Worker + Janitors
        let employee_roles = vec![
            (OrganizationRole::Manager, 2, 4),
            (OrganizationRole::Worker, 5, 10),
            (OrganizationRole::Janitor, 3, 6),
        ];

        for (role, min, max) in employee_roles {
            let count = rng.gen_range(min..=max);
            for _ in 0..count {
                let id = Uuid::new_v4();
                let name = format!(
                    "{} {}",
                    human_names.choose(&mut rng).unwrap(),
                    human_names.choose(&mut rng).unwrap()
                );
                let level = match role {
                    OrganizationRole::Manager => 3,
                    OrganizationRole::Worker => 2,
                    OrganizationRole::Janitor => 1,
                    _ => 1,
                };

                let salary = 50 + level * 25;
                let satisfaction = rng.gen_range(50..=100);
                let productivity = rng.gen_range(60..=100);

                let employee = Entity {
                    id,
                    name,
                    entity_type: EntityType::Human,
                    employment: Some(Employment {
                        organization_id: org_id,
                        role: role.clone(),
                        employee_flags: vec![],
                        level,
                        salary,
                        satisfaction,
                        productivity,
                    }),
                    owner: None,
                    origin: Origin { week_of_birth: rng.gen_range(600..=1500) },
                };
                game_state.entities.insert(id, employee);
                employee_ids.push(id);

                // Give them a pet
                let pet = generate_pet_for_role(&role, week, &mut rng);
                game_state.entities.insert(pet.id, pet);
            }
        }

        // Create organization
        let organization = Organization {
            id: org_id,
            name: org_name,
            vp: Some(vp_id),
            company_relation: CompanyRelation { entity_id: company_id },
            financials: Financials::default(),
            perception: Perception::default(),
            budget: Budget { marketing: 10, rnd: 10, training: 10 },
            initiatives: vec![],
        };

        game_state.organizations.insert(org_id, organization);
    }

    game_state
}

fn generate_pet_for_role(role: &OrganizationRole, current_week: u16, rng: &mut StdRng) -> Entity {
    let entity_type = match role {
        OrganizationRole::VP => {
            let breeds = vec![
                EntityType::Horse(HorseBreed::Arabian),
                EntityType::Dog(DogBreed::GoldenRetriever),
                EntityType::Cat(CatBreed::Bengal),
            ];
            breeds.choose(rng).unwrap().clone()
        }
        OrganizationRole::Manager => {
            let breeds = vec![
                EntityType::Dog(DogBreed::ShibaInu),
                EntityType::Cat(CatBreed::Siamese),
                EntityType::Lizard(LizardBreed::Gecko),
            ];
            breeds.choose(rng).unwrap().clone()
        }
        OrganizationRole::Worker => {
            let breeds = vec![
                EntityType::Fish(FishBreed::Betta),
                EntityType::Cat(CatBreed::Tabby),
                EntityType::Dog(DogBreed::Dachshund),
            ];
            breeds.choose(rng).unwrap().clone()
        }
        OrganizationRole::Janitor | _ => {
            let breeds = vec![
                EntityType::Fish(FishBreed::GoldFish),
                EntityType::Lizard(LizardBreed::BeardedDragon),
            ];
            breeds.choose(rng).unwrap().clone()
        }
    };

    Entity {
        id: Uuid::new_v4(),
        name: format!("{:?}", entity_type),
        entity_type,
        employment: None,
        owner: None,
        origin: Origin {
            week_of_birth: rng.gen_range(10..100),
        },
    }
}

