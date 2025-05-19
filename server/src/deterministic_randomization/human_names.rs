use rand::{rngs::StdRng, seq::SliceRandom};
use shared::{HumanType, OrganizationRole};
use std::collections::{HashMap, VecDeque};

pub fn generate_realistic_name(
    human_type: HumanType,
    role: OrganizationRole,
    rng: &mut StdRng,
) -> String {
    // First name pools by HumanType
    let leader_first_names = ["Alexander", "Victoria", "Michael", "Katherine", "David"];
    let creative_first_names = ["Isabelle", "Julian", "Claire", "Felix", "Sophia"];
    let tech_first_names = ["Ethan", "Chloe", "Marcus", "Leah", "Daniel"];
    let analytical_first_names = ["Nathan", "Laura", "Samuel", "Rebecca", "Charles"];
    let detail_oriented_first_names = ["Simon", "Claire", "Elena", "Lewis", "Tara"];
    let people_person_first_names = ["Amy", "Chris", "Natalie", "Jake", "Melanie"];
    let risk_taker_first_names = ["Logan", "Riley", "Sienna", "Kai", "Jade"];
    let supportive_first_names = ["Hannah", "Noah", "Mia", "Owen", "Lucy"];
    let fast_learner_first_names = ["Ava", "Leo", "Ivy", "Mason", "Lily"];
    let organizer_first_names = ["Eleanor", "Gavin", "Monica", "Dean", "Rachel"];

    // Last name pools by OrganizationRole
    let vp_last_names = ["Armstrong", "Tucker", "Douglas", "Gibson", "Vargas"];
    let cfo_last_names = ["Chambers", "Walsh", "Greene", "Thornton", "Hammond"];
    let coo_last_names = ["Keller", "Walton", "Drake", "Marshall", "Fleming"];
    let software_last_names = ["Nguyen", "Kumar", "Zhao", "Anders", "Lennox"];
    let devops_last_names = ["Briggs", "Hawkins", "Vargas", "Montoya", "Stanford"];
    let marketing_last_names = ["Bishop", "Holland", "Savoy", "Crane", "Banner"];
    let hr_manager_last_names = ["Stevens", "Newton", "Watts", "Lambert", "Yates"];
    let legal_counsel_last_names = ["Blake", "Holmes", "Fitzgerald", "Carter", "Jennings"];
    let accountant_last_names = ["Curtis", "Morris", "Sloan", "Griffith", "Reeves"];
    let data_scientist_last_names = ["Chan", "Klein", "Nguyen", "Wagner", "Mehta"];
    let product_manager_last_names = ["Arnold", "Vega", "Holt", "Francis", "Delaney"];
    let ux_designer_last_names = ["Brady", "Roy", "Quinn", "Hess", "Conway"];
    let warehouse_manager_last_names = ["Boyd", "Weber", "Simon", "Mathews", "Payne"];
    let logistics_coordinator_last_names = ["Hardy", "Gomez", "Fischer", "Ortega", "Jennings"];
    let customer_support_last_names = ["Reed", "Newton", "Mendez", "Lowe", "Parsons"];
    let content_creator_last_names = ["Byrne", "Nash", "Manning", "Bright", "Kerr"];
    let sales_rep_last_names = ["Barber", "Lucas", "Chase", "Adkins", "Sharp"];
    let research_scientist_last_names = ["Lin", "Farrell", "Chang", "McCoy", "Santos"];
    let rnd_engineer_last_names = ["Silva", "Pena", "Dalton", "Ng", "Huang"];

    let first_name_pool = match human_type {
        HumanType::Leader => &leader_first_names,
        HumanType::Creative => &creative_first_names,
        HumanType::TechSavvy => &tech_first_names,
        HumanType::Analytical => &analytical_first_names,
        HumanType::DetailOriented => &detail_oriented_first_names,
        HumanType::PeoplePerson => &people_person_first_names,
        HumanType::RiskTaker => &risk_taker_first_names,
        HumanType::Supportive => &supportive_first_names,
        HumanType::FastLearner => &fast_learner_first_names,
        HumanType::Organizer => &organizer_first_names,
    };

    // Pick last names by Role
    let last_name_pool = match role {
        OrganizationRole::VP => &vp_last_names,
        OrganizationRole::CFO => &cfo_last_names,
        OrganizationRole::COO => &coo_last_names,
        OrganizationRole::HRManager => &hr_manager_last_names,
        OrganizationRole::LegalCounsel => &legal_counsel_last_names,
        OrganizationRole::Accountant => &accountant_last_names,
        OrganizationRole::SoftwareEngineer => &software_last_names,
        OrganizationRole::DataScientist => &data_scientist_last_names,
        OrganizationRole::ProductManager => &product_manager_last_names,
        OrganizationRole::DevOpsEngineer => &devops_last_names,
        OrganizationRole::UXDesigner => &ux_designer_last_names,
        OrganizationRole::WarehouseManager => &warehouse_manager_last_names,
        OrganizationRole::LogisticsCoordinator => &logistics_coordinator_last_names,
        OrganizationRole::CustomerSupport => &customer_support_last_names,
        OrganizationRole::MarketingSpecialist => &marketing_last_names,
        OrganizationRole::ContentCreator => &content_creator_last_names,
        OrganizationRole::SalesRep => &sales_rep_last_names,
        OrganizationRole::ResearchScientist => &research_scientist_last_names,
        OrganizationRole::RnDEngineer => &rnd_engineer_last_names,
    };

    let first_name = first_name_pool.choose(rng).unwrap();
    let last_name = last_name_pool.choose(rng).unwrap();

    format!("{} {}", first_name, last_name)
}

pub fn build_all_human_name_pools(
    rng: &mut StdRng,
) -> HashMap<(HumanType, OrganizationRole), VecDeque<String>> {
    let mut pools = HashMap::new();

    let human_types = [
        HumanType::Analytical,
        HumanType::Creative,
        HumanType::Leader,
        HumanType::DetailOriented,
        HumanType::PeoplePerson,
        HumanType::TechSavvy,
        HumanType::RiskTaker,
        HumanType::Supportive,
        HumanType::FastLearner,
        HumanType::Organizer,
    ];

    let roles = [
        OrganizationRole::VP,
        OrganizationRole::CFO,
        OrganizationRole::COO,
        OrganizationRole::HRManager,
        OrganizationRole::LegalCounsel,
        OrganizationRole::Accountant,
        OrganizationRole::SoftwareEngineer,
        OrganizationRole::DataScientist,
        OrganizationRole::ProductManager,
        OrganizationRole::DevOpsEngineer,
        OrganizationRole::UXDesigner,
        OrganizationRole::WarehouseManager,
        OrganizationRole::LogisticsCoordinator,
        OrganizationRole::CustomerSupport,
        OrganizationRole::MarketingSpecialist,
        OrganizationRole::ContentCreator,
        OrganizationRole::SalesRep,
        OrganizationRole::ResearchScientist,
        OrganizationRole::RnDEngineer,
    ];

    // For each combo, generate a number of unique names
    // (you can adjust the count to suit your game's scale)
    let names_per_pool = 20;

    for &ht in &human_types {
        for &role in &roles {
            let mut name_set = std::collections::HashSet::new();
            let mut names_vec = Vec::new();

            // Generate names until you have the desired count or hit a max attempts limit to avoid infinite loops
            let max_attempts = 100;
            let mut attempts = 0;
            while name_set.len() < names_per_pool && attempts < max_attempts {
                let name = generate_realistic_name(ht, role, rng);
                if !name_set.contains(&name) {
                    name_set.insert(name.clone());
                    names_vec.push(name);
                }
                attempts += 1;
            }

            names_vec.shuffle(rng);
            pools.insert((ht, role), VecDeque::from(names_vec));
        }
    }

    pools
}
