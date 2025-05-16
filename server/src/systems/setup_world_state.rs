use bevy::prelude::Commands;
use shared::{
    CatBreed, Company, DogBreed, Employed, EmployeeFlag, EmployeeFlags, EntityType, HorseBreed,
    InternalEntity, Level, LizardBreed, Money, Name, OrgRole, Organization, Owner, Player,
    Productivity, Reputation, Salary, Satisfaction, Type, Week,
};
use tracing::info;
use uuid::Uuid;

pub fn setup_world_state(mut commands: Commands) {
    info!("spawning player");
    commands.spawn((Player, Money(1_000_000), Reputation(50), Week(1)));

    info!("spawning organizations");
    let org1_id = Uuid::from_u128(1);
    let org2_id = Uuid::from_u128(2);

    commands.spawn(Organization {
        id: org1_id,
        name: "Red Division".into(),
        vp: Some(Uuid::from_u128(101)),
    });

    commands.spawn(Organization {
        id: org2_id,
        name: "Blue Division".into(),
        vp: Some(Uuid::from_u128(201)),
    });

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
    ));

    info!("spawning Company");
    commands.insert_resource(Company {
        revenue: 0,
        public_opinion: 50,
    });
}
