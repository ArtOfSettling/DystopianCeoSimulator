use rand::Rng;
use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use shared::{HumanType, OrganizationRole};

fn human_types_for_role(organization_role: &OrganizationRole) -> Vec<(HumanType, u32)> {
    match organization_role {
        // Core / Corporate
        OrganizationRole::VP => vec![
            (HumanType::Leader, 40),
            (HumanType::RiskTaker, 25),
            (HumanType::Analytical, 15),
            (HumanType::PeoplePerson, 10),
            (HumanType::Organizer, 10),
        ],
        OrganizationRole::CFO => vec![
            (HumanType::Analytical, 40),
            (HumanType::DetailOriented, 30),
            (HumanType::Organizer, 15),
            (HumanType::Leader, 10),
            (HumanType::RiskTaker, 5),
        ],
        OrganizationRole::COO => vec![
            (HumanType::Organizer, 35),
            (HumanType::Leader, 30),
            (HumanType::DetailOriented, 20),
            (HumanType::Analytical, 10),
            (HumanType::Supportive, 5),
        ],
        OrganizationRole::HRManager => vec![
            (HumanType::PeoplePerson, 40),
            (HumanType::Organizer, 25),
            (HumanType::Supportive, 15),
            (HumanType::DetailOriented, 10),
            (HumanType::FastLearner, 10),
        ],
        OrganizationRole::LegalCounsel => vec![
            (HumanType::Analytical, 40),
            (HumanType::DetailOriented, 30),
            (HumanType::Supportive, 10),
            (HumanType::Organizer, 10),
            (HumanType::RiskTaker, 10),
        ],
        OrganizationRole::Accountant => vec![
            (HumanType::DetailOriented, 45),
            (HumanType::Analytical, 35),
            (HumanType::Organizer, 15),
            (HumanType::Supportive, 5),
        ],

        // Tech & Product
        OrganizationRole::SoftwareEngineer => vec![
            (HumanType::TechSavvy, 50),
            (HumanType::Analytical, 25),
            (HumanType::DetailOriented, 15),
            (HumanType::FastLearner, 10),
        ],
        OrganizationRole::DataScientist => vec![
            (HumanType::Analytical, 50),
            (HumanType::TechSavvy, 30),
            (HumanType::Creative, 10),
            (HumanType::FastLearner, 10),
        ],
        OrganizationRole::ProductManager => vec![
            (HumanType::Leader, 35),
            (HumanType::Organizer, 25),
            (HumanType::Creative, 20),
            (HumanType::PeoplePerson, 15),
            (HumanType::FastLearner, 5),
        ],
        OrganizationRole::DevOpsEngineer => vec![
            (HumanType::TechSavvy, 45),
            (HumanType::Analytical, 30),
            (HumanType::DetailOriented, 15),
            (HumanType::Supportive, 10),
        ],
        OrganizationRole::UXDesigner => vec![
            (HumanType::Creative, 50),
            (HumanType::PeoplePerson, 25),
            (HumanType::FastLearner, 15),
            (HumanType::TechSavvy, 10),
        ],

        // Operations
        OrganizationRole::WarehouseManager => vec![
            (HumanType::Organizer, 40),
            (HumanType::Leader, 25),
            (HumanType::DetailOriented, 20),
            (HumanType::Supportive, 15),
        ],
        OrganizationRole::LogisticsCoordinator => vec![
            (HumanType::Organizer, 40),
            (HumanType::DetailOriented, 25),
            (HumanType::Supportive, 20),
            (HumanType::FastLearner, 15),
        ],
        OrganizationRole::CustomerSupport => vec![
            (HumanType::PeoplePerson, 50),
            (HumanType::Supportive, 30),
            (HumanType::FastLearner, 20),
        ],

        // Marketing & Sales
        OrganizationRole::MarketingSpecialist => vec![
            (HumanType::Creative, 40),
            (HumanType::PeoplePerson, 30),
            (HumanType::RiskTaker, 15),
            (HumanType::FastLearner, 15),
        ],
        OrganizationRole::ContentCreator => vec![
            (HumanType::Creative, 60),
            (HumanType::FastLearner, 25),
            (HumanType::PeoplePerson, 10),
            (HumanType::TechSavvy, 5),
        ],
        OrganizationRole::SalesRep => vec![
            (HumanType::PeoplePerson, 50),
            (HumanType::RiskTaker, 30),
            (HumanType::FastLearner, 20),
        ],

        // Research
        OrganizationRole::ResearchScientist => vec![
            (HumanType::Analytical, 50),
            (HumanType::DetailOriented, 25),
            (HumanType::Creative, 15),
            (HumanType::FastLearner, 10),
        ],
        OrganizationRole::RnDEngineer => vec![
            (HumanType::Creative, 40),
            (HumanType::TechSavvy, 30),
            (HumanType::Analytical, 20),
            (HumanType::FastLearner, 10),
        ],
    }
}

fn weighted_sample<T: Clone>(choices: &[(T, u32)], rng: &mut impl Rng) -> Option<T> {
    let weights: Vec<u32> = choices.iter().map(|c| c.1).collect();
    let dist = WeightedIndex::new(&weights).ok()?;
    Some(choices[dist.sample(rng)].0.clone())
}

pub fn generate_human_type_for_organization_role(
    role: &OrganizationRole,
    rng: &mut impl Rng,
) -> Option<HumanType> {
    let weighted_humans = human_types_for_role(role);
    weighted_sample(&weighted_humans, rng)
}
