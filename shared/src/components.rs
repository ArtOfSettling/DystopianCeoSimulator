use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Player;

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Money(pub f64);

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Reputation(pub i32);

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Week(pub u32);

#[derive(Component)]
pub struct Employee {
    pub id: Uuid,
    pub name: String,
    pub role: String,
}

#[derive(Component)]
pub struct Satisfaction(pub f64);

#[derive(Component)]
pub struct Productivity(pub f64);

#[derive(Component)]
pub struct Salary(pub f64);

#[derive(Component)]
pub struct EmployeeFlags(pub Vec<EmployeeFlag>);

#[derive(Debug, Clone)]
pub enum EmployeeFlag {
    WantsRaise,
    BurnedOut,
    Loyal,
}

#[derive(Component, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalEntity(Uuid);

impl InternalEntity {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}
