use rand::Rng;
use rand::rngs::StdRng;
use shared::{CORE_ORGANIZATION_TYPES, CompanyType, OrganizationType};
use std::collections::HashSet;

fn get_weighted_organization_for_company_type(
    company_type: &CompanyType,
) -> Vec<(OrganizationType, f64)> {
    match company_type {
        CompanyType::ECommerce => vec![
            (OrganizationType::Warehouse, 0.3),
            (OrganizationType::RetailSite, 0.2),
            (OrganizationType::SupportCenter, 0.1),
            (OrganizationType::MarketingTeam, 0.1),
            (OrganizationType::LogisticsHub, 0.1),
            (OrganizationType::ProductManagement, 0.05),
            (OrganizationType::ITInfrastructure, 0.05),
            (OrganizationType::DataAnalytics, 0.05),
            (OrganizationType::RnD, 0.03),
            (OrganizationType::ContentCreation, 0.02),
        ],
    }
}

pub fn generate_organization_types_for_company(
    rng: &mut StdRng,
    company_type: &CompanyType,
    count: usize,
) -> Vec<OrganizationType> {
    let mut selected_organizations: HashSet<OrganizationType> =
        CORE_ORGANIZATION_TYPES.iter().copied().collect();
    let weighted_organizations = get_weighted_organization_for_company_type(company_type);
    let total_weight: f64 = weighted_organizations.iter().map(|(_, w)| *w).sum();

    while selected_organizations.len() < count {
        let pick = rng.gen_range(0.0..total_weight);
        let mut accumulator = 0.0;

        for (org_type, weight) in &weighted_organizations {
            accumulator += weight;
            if pick <= accumulator {
                selected_organizations.insert(*org_type);
                break;
            }
        }

        // Avoid infinite loops if count > total possible orgs
        if selected_organizations.len()
            == weighted_organizations.len() + CORE_ORGANIZATION_TYPES.len()
        {
            break;
        }
    }

    selected_organizations.into_iter().collect()
}
