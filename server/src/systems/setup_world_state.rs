use bevy::prelude::Commands;
use shared::{
    CatBreed, Company, DogBreed, Employed, EmployeeFlag, EmployeeFlags, EntityType, Financials,
    HorseBreed, InternalEntity, Level, LizardBreed, Money, Name, OrgBudget, OrgInitiative, OrgRole,
    Organization, Owner, Player, Productivity, PublicOpinion, Reputation, Salary, Satisfaction,
    Type, Week, WeekOfBirth,
};
use tracing::info;
use uuid::Uuid;

pub fn setup_world_state(mut commands: Commands) {
    info!("spawning player");
    commands.spawn((
        Player,
        Money(1_000_000),
        Reputation(50),
        PublicOpinion(50),
        Week(1),
    ));

    info!("spawning organizations");
    let org1_id = Uuid::from_u128(1);
    let org2_id = Uuid::from_u128(2);

    let starting_week = 0i32;

    commands.spawn((
        Organization {
            id: org1_id,
            name: "Red Division".into(),
            vp: Some(Uuid::from_u128(101)),
            initiatives: vec![
                OrgInitiative::Marketing {
                    weeks_remaining: 52,
                },
                OrgInitiative::RnD {
                    weeks_remaining: 29,
                },
            ],
            financials: Financials {
                this_weeks_income: 0,
                this_weeks_expenses: 0,
                this_weeks_net_profit: 0,
                actual_cash: 256_227,
            },
            budget: OrgBudget {
                marketing: 100,
                rnd: 100,
                training: 100,
            },
        },
        Reputation(50),
        PublicOpinion(50),
    ));

    commands.spawn((
        Organization {
            id: org2_id,
            name: "Blue Division".into(),
            vp: Some(Uuid::from_u128(201)),
            initiatives: vec![OrgInitiative::Training {
                weeks_remaining: 11,
            }],
            financials: Financials {
                this_weeks_income: 0,
                this_weeks_expenses: 0,
                this_weeks_net_profit: 0,
                actual_cash: 709_991,
            },
            budget: OrgBudget {
                marketing: 100,
                rnd: 100,
                training: 100,
            },
        },
        Reputation(50),
        PublicOpinion(50),
    ));

    info!("spawning vp of Red Division");
    let alice_id = Uuid::from_u128(101);
    commands.spawn((
        InternalEntity { id: alice_id },
        Name("Alice".into()),
        Type(EntityType::Human),
        Employed {
            owner_id: org1_id,
            role: OrgRole::VP,
        },
        Level(10_000),
        Satisfaction(85),
        Productivity(90),
        Salary(12_000),
        WeekOfBirth(starting_week.saturating_sub(57 * 52 + 2 * 4 + 6)), // 57 years old
        EmployeeFlags(vec![EmployeeFlag::Loyal]),
    ));

    info!("spawning vp of Red Division (children and pets)");
    commands.spawn((
        InternalEntity {
            id: Uuid::from_u128(1001),
        },
        Name("Little Al".into()),
        Type(EntityType::Human),
        Owner {
            owner_id: Some(alice_id),
        },
        WeekOfBirth(starting_week.saturating_sub(8 * 52 + 9 * 6 + 6)), // 8 years old
    ));

    commands.spawn((
        InternalEntity {
            id: Uuid::from_u128(1002),
        },
        Name("Mittens".into()),
        Type(EntityType::Cat(CatBreed::Siamese)),
        Owner {
            owner_id: Some(alice_id),
        },
        WeekOfBirth(starting_week.saturating_sub(52 + 4 + 1)), // 1 year old
    ));

    info!("spawning employees of Red Division");
    commands.spawn((
        InternalEntity {
            id: Uuid::from_u128(102),
        },
        Name("Bob".into()),
        Type(EntityType::Human),
        Employed {
            owner_id: org1_id,
            role: OrgRole::Employee,
        },
        Level(10_000),
        Satisfaction(70),
        Productivity(75),
        Salary(8_000),
        EmployeeFlags(vec![]),
        WeekOfBirth(starting_week.saturating_sub(31 * 52)), // 31 years old
    ));

    commands.spawn((
        InternalEntity {
            id: Uuid::from_u128(103),
        },
        Name("Charlie".into()),
        Type(EntityType::Human),
        Employed {
            owner_id: org1_id,
            role: OrgRole::Employee,
        },
        Level(10_000),
        Satisfaction(60),
        Productivity(65),
        Salary(7_500),
        EmployeeFlags(vec![EmployeeFlag::WantsRaise]),
        WeekOfBirth(starting_week.saturating_sub(27 * 52)), // 27 years old
    ));

    info!("spawning vp of Blue Division");
    let diana_id = Uuid::from_u128(201);
    commands.spawn((
        InternalEntity {
            id: Uuid::from_u128(201),
        },
        Name("Diana".into()),
        Type(EntityType::Human),
        Employed {
            owner_id: org2_id,
            role: OrgRole::VP,
        },
        Level(10_000),
        Satisfaction(90),
        Productivity(95),
        Salary(13_000),
        EmployeeFlags(vec![]),
        WeekOfBirth(starting_week.saturating_sub(89 * 52 + 10 * 4 + 10)), // 89 years old
    ));

    info!("spawning Diana's pet");
    commands.spawn((
        InternalEntity {
            id: Uuid::from_u128(2001),
        },
        Name("Doge".into()),
        Type(EntityType::Dog(DogBreed::ShibaInu)),
        Owner {
            owner_id: Some(diana_id),
        },
        WeekOfBirth(starting_week.saturating_sub(7 * 52 + 4 * 4 + 2)), // 7 years old
    ));

    commands.spawn((
        InternalEntity {
            id: Uuid::from_u128(2002),
        },
        Name("Horsy".into()),
        Type(EntityType::Horse(HorseBreed::Arabian)),
        Owner {
            owner_id: Some(diana_id),
        },
        WeekOfBirth(starting_week.saturating_sub(4 * 52 + 8 * 4 + 1)), // 4 years old
    ));

    commands.spawn((
        InternalEntity {
            id: Uuid::from_u128(2003),
        },
        Name("Horse Face".into()),
        Type(EntityType::Horse(HorseBreed::Clydesdale)),
        Owner {
            owner_id: Some(diana_id),
        },
        WeekOfBirth(starting_week.saturating_sub(9 * 52)), // 9 years old
    ));

    info!("spawning employees of Blue Division");
    commands.spawn((
        InternalEntity {
            id: Uuid::from_u128(202),
        },
        Name("Eli".into()),
        Type(EntityType::Human),
        Employed {
            owner_id: org2_id,
            role: OrgRole::Employee,
        },
        Level(10_000),
        Satisfaction(75),
        Productivity(70),
        Salary(6_500),
        EmployeeFlags(vec![]),
        WeekOfBirth(starting_week.saturating_sub(32 * 52)), // 32 years old
    ));

    let faye_id = Uuid::from_u128(203);
    commands.spawn((
        InternalEntity { id: faye_id },
        Name("Faye".into()),
        Type(EntityType::Human),
        Employed {
            owner_id: org2_id,
            role: OrgRole::Employee,
        },
        Level(10_000),
        Satisfaction(80),
        Productivity(80),
        Salary(7_000),
        EmployeeFlags(vec![EmployeeFlag::BurnedOut]),
        WeekOfBirth(starting_week.saturating_sub(39 * 52)), // 39 years old
    ));

    commands.spawn((
        InternalEntity {
            id: Uuid::from_u128(2004),
        },
        Name("Beardy".into()),
        Type(EntityType::Lizard(LizardBreed::BeardedDragon)),
        Owner {
            owner_id: Some(faye_id),
        },
        WeekOfBirth(starting_week.saturating_sub(2 * 52)), // 2 years old
    ));

    info!("spawning Company");
    commands.insert_resource(Company {
        public_opinion: 50,
        reputation: 50,
        financials: Financials {
            this_weeks_income: 0,
            this_weeks_expenses: 0,
            this_weeks_net_profit: 0,
            actual_cash: 1_102_101,
        },
    });
}
