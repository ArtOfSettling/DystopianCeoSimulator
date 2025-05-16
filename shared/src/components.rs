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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntityType {
    Human,
    Cat(CatBreed),
    Dog(DogBreed),
    Horse(HorseBreed),
    Lizard(LizardBreed),
    Fish(FishBreed),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CatBreed {
    Tabby,
    Siamese,
    Persian,
    MaineCoon,
    Sphynx,
    ScottishFold,
    Bengal,
    Ragdoll,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DogBreed {
    ShibaInu,
    LabradorRetriever,
    Poodle,
    Bulldog,
    GermanShepherd,
    Dachshund,
    GoldenRetriever,
    Chihuahua,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HorseBreed {
    Appaloosa,
    Arabian,
    Clydesdale,
    Thoroughbred,
    Mustang,
    ShetlandPony,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LizardBreed {
    BeardedDragon,
    Gecko,
    Iguana,
    Chameleon,
    Monitor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FishBreed {
    GoldFish,
    Guppy,
    Betta,
    Angelfish,
    Tetra,
    Clownfish,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub vp: Option<Uuid>,
    pub financials: Financials,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Financials {
    pub actual_cash: i32,
    pub this_weeks_income: i32,
    pub this_weeks_expenses: i32,
    pub this_weeks_net_profit: i32,
}

#[derive(Component)]
pub struct Employed {
    pub owner_id: Uuid,
    pub role: OrgRole,
}

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Type(pub EntityType);

#[derive(Component)]
pub struct Owner {
    pub owner_id: Option<Uuid>,
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
pub struct WeekOfBirth(pub i32);

#[derive(Component)]
pub struct EmployeeFlags(pub Vec<EmployeeFlag>);

#[derive(Debug, Clone)]
pub enum EmployeeFlag {
    WantsRaise,
    BurnedOut,
    Loyal,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct InternalEntity {
    pub id: Uuid,
}
