use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Player;

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Money(pub i32);

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Reputation(pub i32);

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Week(pub u32);

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub vp: Option<Uuid>, // Employee ID
}

#[derive(Component)]
pub struct Employee {
    pub id: Uuid,
    pub name: String,
    pub role: String,
    pub employment_status: EmploymentStatus,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationMember {
    pub organization_id: Uuid,
    pub role: OrgRole,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrgRole {
    VP,
    Employee,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EmploymentStatus {
    Active,
    Fired,
    Quit,
}

#[derive(Component)]
pub struct Satisfaction(pub i32);

#[derive(Component)]
pub struct Level(pub u32);

#[derive(Component)]
pub struct Productivity(pub i32);

#[derive(Component)]
pub struct Salary(pub i32);

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
