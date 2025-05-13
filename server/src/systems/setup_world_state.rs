use bevy::prelude::Commands;
use rand::Rng;
use rand::seq::SliceRandom;
use shared::{
    Company, Employee, EmployeeFlags, EmploymentStatus, Money, Player, Productivity, Reputation,
    Salary, Satisfaction, Week,
};
use tracing::info;
use uuid::Uuid;

pub fn setup_world_state(mut commands: Commands) {
    info!("spawning player");
    commands.spawn((Player, Money(1_000_000), Reputation(50), Week(1)));

    commands.insert_resource(Company {
        revenue: 0,
        public_opinion: 50,
    });

    let names = vec!["Alex", "Jordan", "Morgan", "Taylor", "Riley"];
    let roles = vec!["Engineer", "HR", "Sales", "IT", "Legal"];

    let mut rng = rand::thread_rng();

    for _ in 0..3 {
        let name = names.choose(&mut rng).unwrap().to_string();
        let role = roles.choose(&mut rng).unwrap().to_string();
        let salary = rng.gen_range(40_000..80_000);
        let satisfaction = rng.gen_range(40..80);
        let productivity = rng.gen_range(30..90);

        commands.spawn((
            Employee {
                id: Uuid::new_v4(),
                name,
                role,
                employment_status: EmploymentStatus::Active,
            },
            Salary(salary),
            Satisfaction(satisfaction),
            Productivity(productivity),
            EmployeeFlags(vec![]),
        ));
    }

    info!("Startup complete: Player + 3 employees spawned.");
}
