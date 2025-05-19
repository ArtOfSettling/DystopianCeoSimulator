use crate::deterministic_randomization::employee_types::weighted_employee_types_for_org;
use crate::deterministic_randomization::generate_human_type_for_organization_role;
use rand::prelude::*;
use shared::OrganizationRole::{
    Accountant, CFO, COO, ContentCreator, CustomerSupport, DataScientist, DevOpsEngineer,
    HRManager, LegalCounsel, LogisticsCoordinator, MarketingSpecialist, ProductManager,
    ResearchScientist, RnDEngineer, SalesRep, SoftwareEngineer, UXDesigner, VP, WarehouseManager,
};
use shared::{HumanType, OrganizationRole, OrganizationType};

pub struct Employee {
    pub role: OrganizationRole,
    pub human_type: HumanType,
    pub rank: u32,
}

pub struct OrganizationChart {
    pub organization_type: OrganizationType,
    pub employees: Vec<Employee>,
}

pub fn generate_organization_chart(
    org_type: OrganizationType,
    rng: &mut StdRng,
    min_size: usize,
    max_size: usize,
) -> OrganizationChart {
    let num_employees = rng.gen_range(min_size..=max_size);
    let weighted_roles = weighted_employee_types_for_org(org_type);

    let roles: Vec<_> = weighted_roles.iter().map(|(r, _w)| r).collect();
    let weights: Vec<_> = weighted_roles.iter().map(|(_r, w)| *w).collect();
    let dist = rand::distributions::WeightedIndex::new(&weights).unwrap();

    let mut employees = Vec::new();

    for _ in 0..num_employees {
        let role = roles[dist.sample(rng)];
        let human_type = generate_human_type_for_organization_role(role, rng).unwrap();
        let rank = match role {
            VP => 0,
            CFO | COO => 1,
            HRManager | LegalCounsel | Accountant | ProductManager => 2,
            SoftwareEngineer | DataScientist | DevOpsEngineer | UXDesigner
            | MarketingSpecialist | ContentCreator | SalesRep | WarehouseManager
            | LogisticsCoordinator | CustomerSupport | ResearchScientist | RnDEngineer => 3,
        };

        employees.push(Employee {
            role: *role,
            human_type,
            rank,
        });
    }

    OrganizationChart {
        organization_type: org_type,
        employees,
    }
}
