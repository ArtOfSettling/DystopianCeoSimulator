use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub week: u16,
    pub players: Vec<Player>,
    pub companies: HashMap<Uuid, Company>,
    pub organizations: HashMap<Uuid, Organization>,
    pub entities: HashMap<Uuid, Entity>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Employment {
    pub organization_id: Uuid,
    pub role: OrganizationRole,
    pub employee_flags: Vec<EmployeeFlag>,
    pub level: u16,
    pub salary: u16,
    pub satisfaction: u16,
    pub productivity: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompanyRelation {
    pub entity_id: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Owner {
    pub entity_id: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Origin {
    pub week_of_birth: i16,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Financials {
    pub actual_cash: i32,
    pub this_weeks_income: i32,
    pub this_weeks_expenses: i32,
    pub this_weeks_net_profit: i32,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Perception {
    pub public_opinion: i16,
    pub reputation: i16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Budget {
    pub marketing: u16,
    pub rnd: u16,
    pub training: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub company_type: CompanyType,
    pub perception: Perception,
    pub financials: Financials,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: Option<Uuid>,
    pub financials: Financials,
    pub perception: Perception,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub organization_type: OrganizationType,
    pub vp: Option<Uuid>,
    pub company_relation: CompanyRelation,
    pub financials: Financials,
    pub perception: Perception,
    pub budget: Budget,
    pub initiatives: Vec<Initiative>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: Uuid,
    pub entity_type: EntityType,
    pub name: String,
    pub employment: Option<Employment>,
    pub owner: Option<Owner>,
    pub origin: Origin,
    pub flags: Vec<EntityFlag>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CompanyType {
    ECommerce,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, PartialEq, Eq, Hash)]
pub enum OrganizationType {
    Warehouse,
    RetailSite,
    SupportCenter,
    MarketingTeam,
    LogisticsHub,
    ProductManagement,
    ITInfrastructure,
    Finance,
    HR,
    Legal,
    DataAnalytics,
    RnD,
    ContentCreation,
}

pub const CORE_ORGANIZATION_TYPES: &[OrganizationType] = &[
    OrganizationType::Legal,
    OrganizationType::HR,
    OrganizationType::Finance,
];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EntityFlag {
    Hoarder,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EmployeeFlag {
    WantsRaise,
    BurnedOut,
    Loyal,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrganizationRole {
    // Core / Corporate
    VP,
    CFO,
    COO,
    HRManager,
    LegalCounsel,
    Accountant,

    // Tech & Product
    SoftwareEngineer,
    DataScientist,
    ProductManager,
    DevOpsEngineer,
    UXDesigner,

    // Operations
    WarehouseManager,
    LogisticsCoordinator,
    CustomerSupport,

    // Marketing & Sales
    MarketingSpecialist,
    ContentCreator,
    SalesRep,

    // Research
    ResearchScientist,
    RnDEngineer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EmploymentStatus {
    Active,
    Fired,
    Quit,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub enum Initiative {
    Marketing { weeks_remaining: u16 },
    Training { weeks_remaining: u16 },
    RnD { weeks_remaining: u16 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntityType {
    Human(HumanType),
    Cat(CatBreed),
    Dog(DogBreed),
    Horse(HorseBreed),
    Lizard(LizardBreed),
    Fish(FishBreed),
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HumanType {
    Analytical,
    Creative,
    Leader,
    DetailOriented,
    PeoplePerson,
    TechSavvy,
    RiskTaker,
    Supportive,
    FastLearner,
    Organizer,
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
