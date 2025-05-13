use bevy::prelude::Commands;
use shared::{
    Company, Employee, EmployeeFlag, EmployeeFlags, EmploymentStatus, Money, OrgRole, Organization,
    OrganizationMember, Player, Productivity, Reputation, Salary, Satisfaction, Week,
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
    commands.spawn((
        Employee {
            id: Uuid::from_u128(101),
            name: "Alice".into(),
            role: "VP of Red".into(),
            employment_status: EmploymentStatus::Active,
        },
        OrganizationMember {
            organization_id: org1_id,
            role: OrgRole::VP,
        },
        Satisfaction(85),
        Productivity(90),
        Salary(12_000),
        EmployeeFlags(vec![EmployeeFlag::Loyal]),
    ));

    info!("spawning employees of Red Division");
    commands.spawn((
        Employee {
            id: Uuid::from_u128(102),
            name: "Bob".into(),
            role: "Engineer".into(),
            employment_status: EmploymentStatus::Active,
        },
        OrganizationMember {
            organization_id: org1_id,
            role: OrgRole::Employee,
        },
        Satisfaction(70),
        Productivity(75),
        Salary(8_000),
        EmployeeFlags(vec![]),
    ));

    commands.spawn((
        Employee {
            id: Uuid::from_u128(103),
            name: "Charlie".into(),
            role: "Designer".into(),
            employment_status: EmploymentStatus::Active,
        },
        OrganizationMember {
            organization_id: org1_id,
            role: OrgRole::Employee,
        },
        Satisfaction(60),
        Productivity(65),
        Salary(7_500),
        EmployeeFlags(vec![EmployeeFlag::WantsRaise]),
    ));

    info!("spawning vp of Blue Division");
    commands.spawn((
        Employee {
            id: Uuid::from_u128(201),
            name: "Diana".into(),
            role: "VP of Blue".into(),
            employment_status: EmploymentStatus::Active,
        },
        OrganizationMember {
            organization_id: org2_id,
            role: OrgRole::VP,
        },
        Satisfaction(90),
        Productivity(95),
        Salary(13_000),
        EmployeeFlags(vec![]),
    ));

    info!("spawning employees of Blue Division");
    commands.spawn((
        Employee {
            id: Uuid::from_u128(202),
            name: "Eli".into(),
            role: "QA Analyst".into(),
            employment_status: EmploymentStatus::Active,
        },
        OrganizationMember {
            organization_id: org2_id,
            role: OrgRole::Employee,
        },
        Satisfaction(75),
        Productivity(70),
        Salary(6_500),
        EmployeeFlags(vec![]),
    ));

    commands.spawn((
        Employee {
            id: Uuid::from_u128(203),
            name: "Faye".into(),
            role: "Marketing".into(),
            employment_status: EmploymentStatus::Active,
        },
        OrganizationMember {
            organization_id: org2_id,
            role: OrgRole::Employee,
        },
        Satisfaction(80),
        Productivity(80),
        Salary(7_000),
        EmployeeFlags(vec![EmployeeFlag::BurnedOut]),
    ));

    info!("spawning Company");
    commands.insert_resource(Company {
        revenue: 0,
        public_opinion: 50,
    });
}
