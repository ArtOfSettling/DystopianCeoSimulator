use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use shared::OrganizationType;
use std::collections::{HashMap, VecDeque};

fn generate_all_org_names(organization_type: OrganizationType) -> Vec<String> {
    match organization_type {
        OrganizationType::Warehouse => {
            let prefixes = ["Central", "Prime", "North", "East", "Global"];
            let suffixes = ["Warehouse", "Depot", "Storage", "Hub"];
            prefixes
                .iter()
                .flat_map(|p| suffixes.iter().map(move |s| format!("{} {}", p, s)))
                .collect()
        }
        OrganizationType::RetailSite => {
            let adjectives = ["Bright", "Cozy", "Urban", "Metro", "Sunny"];
            let nouns = ["Mart", "Outlet", "Shop", "Store", "Bazaar"];
            adjectives
                .iter()
                .flat_map(|a| nouns.iter().map(move |n| format!("{} {}", a, n)))
                .collect()
        }
        OrganizationType::SupportCenter => {
            let cores = ["Help", "Support", "Care", "Assist", "Resolve"];
            let suffixes = ["Center", "Desk", "Hub"];
            cores
                .iter()
                .flat_map(|c| suffixes.iter().map(move |s| format!("{} {}", c, s)))
                .collect()
        }
        OrganizationType::MarketingTeam => {
            let adjectives = ["Creative", "Bright", "Vivid", "Bold", "Fresh"];
            let nouns = ["Minds", "Vision", "Hive", "Squad"];
            adjectives
                .iter()
                .flat_map(|a| nouns.iter().map(move |n| format!("{} {}", a, n)))
                .collect()
        }
        OrganizationType::LogisticsHub => {
            let prefixes = ["Rapid", "Swift", "Dynamic", "Global", "Prime"];
            let suffixes = ["Logistics", "Transit", "Distribution", "Hub"];
            prefixes
                .iter()
                .flat_map(|p| suffixes.iter().map(move |s| format!("{} {}", p, s)))
                .collect()
        }
        OrganizationType::ProductManagement => {
            let nouns = ["Product", "Vision", "Strategy", "Pipeline", "Roadmap"];
            let suffixes = ["Team", "Group", "Board"];
            nouns
                .iter()
                .flat_map(|n| suffixes.iter().map(move |s| format!("{} {}", n, s)))
                .collect()
        }
        OrganizationType::ITInfrastructure => {
            let modifiers = ["Cyber", "Tech", "Net", "Cloud", "Data"];
            let nouns = ["Ops", "Systems", "Grid", "Core"];
            modifiers
                .iter()
                .flat_map(|m| nouns.iter().map(move |n| format!("{} {}", m, n)))
                .collect()
        }
        OrganizationType::Finance => {
            let adjectives = ["Prime", "Capital", "Fortune", "Legacy", "Summit"];
            let nouns = ["Finance", "Holdings", "Investments", "Group"];
            adjectives
                .iter()
                .flat_map(|a| nouns.iter().map(move |n| format!("{} {}", a, n)))
                .collect()
        }
        OrganizationType::HR => {
            let adjectives = ["People", "Talent", "Bright", "Core", "Unity"];
            let nouns = ["Resources", "Relations", "Management", "Team"];
            adjectives
                .iter()
                .flat_map(|a| nouns.iter().map(move |n| format!("{} {}", a, n)))
                .collect()
        }
        OrganizationType::Legal => {
            let prefixes = ["Prime", "Legacy", "Summit", "Cornerstone"];
            let suffixes = ["Legal", "Counsel", "Advisors", "Partners"];
            prefixes
                .iter()
                .flat_map(|p| suffixes.iter().map(move |s| format!("{} {}", p, s)))
                .collect()
        }
        OrganizationType::DataAnalytics => {
            let prefixes = ["Data", "Insight", "Core", "Pulse", "Metric"];
            let suffixes = ["Analytics", "Labs", "Solutions", "Systems"];
            prefixes
                .iter()
                .flat_map(|p| suffixes.iter().map(move |s| format!("{} {}", p, s)))
                .collect()
        }
        OrganizationType::RnD => {
            let prefixes = ["Innovate", "NextGen", "Pioneer", "Quantum", "Vision"];
            let suffixes = ["Labs", "Research", "Development", "Center"];
            prefixes
                .iter()
                .flat_map(|p| suffixes.iter().map(move |s| format!("{} {}", p, s)))
                .collect()
        }
        OrganizationType::ContentCreation => {
            let adjectives = ["Creative", "Bright", "Bold", "Fresh", "Dynamic"];
            let nouns = ["Studios", "Works", "Labs", "Media"];
            adjectives
                .iter()
                .flat_map(|a| nouns.iter().map(move |n| format!("{} {}", a, n)))
                .collect()
        }
    }
}

pub fn build_all_org_name_pools(rng: &mut StdRng) -> HashMap<OrganizationType, VecDeque<String>> {
    let mut map = HashMap::new();

    for &org_type in &[
        OrganizationType::Warehouse,
        OrganizationType::RetailSite,
        OrganizationType::SupportCenter,
        OrganizationType::MarketingTeam,
        OrganizationType::LogisticsHub,
        OrganizationType::ProductManagement,
        OrganizationType::ITInfrastructure,
        OrganizationType::Finance,
        OrganizationType::HR,
        OrganizationType::Legal,
        OrganizationType::DataAnalytics,
        OrganizationType::RnD,
        OrganizationType::ContentCreation,
    ] {
        let mut names = generate_all_org_names(org_type);
        names.shuffle(rng);
        map.insert(org_type, VecDeque::from(names));
    }

    map
}
