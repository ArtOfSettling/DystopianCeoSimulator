use shared::{OrganizationRole, OrganizationType};

pub fn weighted_employee_types_for_org(org_type: OrganizationType) -> Vec<(OrganizationRole, u32)> {
    use OrganizationRole::*;
    use OrganizationType::*;

    match org_type {
        Warehouse => vec![
            (WarehouseManager, 40),
            (LogisticsCoordinator, 40),
            (CustomerSupport, 20),
        ],
        RetailSite => vec![
            (CustomerSupport, 50),
            (SalesRep, 30),
            (MarketingSpecialist, 10),
            (HRManager, 10),
        ],
        SupportCenter => vec![
            (CustomerSupport, 60),
            (HRManager, 15),
            (LegalCounsel, 5),
            (Accountant, 5),
            (SalesRep, 15),
        ],
        MarketingTeam => vec![
            (MarketingSpecialist, 50),
            (ContentCreator, 30),
            (SalesRep, 20),
        ],
        LogisticsHub => vec![
            (LogisticsCoordinator, 50),
            (WarehouseManager, 30),
            (CustomerSupport, 10),
            (HRManager, 10),
        ],
        ProductManagement => vec![
            (ProductManager, 60),
            (SoftwareEngineer, 25),
            (UXDesigner, 10),
            (DataScientist, 5),
        ],
        ITInfrastructure => vec![
            (DevOpsEngineer, 50),
            (SoftwareEngineer, 30),
            (DataScientist, 10),
            (HRManager, 5),
            (LegalCounsel, 5),
        ],
        Finance => vec![
            (Accountant, 60),
            (CFO, 10),
            (HRManager, 10),
            (LegalCounsel, 10),
            (CustomerSupport, 10),
        ],
        HR => vec![(HRManager, 80), (LegalCounsel, 10), (CustomerSupport, 10)],
        Legal => vec![(LegalCounsel, 80), (HRManager, 10), (Accountant, 10)],
        DataAnalytics => vec![
            (DataScientist, 70),
            (ResearchScientist, 20),
            (SoftwareEngineer, 10),
        ],
        RnD => vec![
            (RnDEngineer, 50),
            (ResearchScientist, 30),
            (SoftwareEngineer, 10),
            (ProductManager, 10),
        ],
        ContentCreation => vec![
            (ContentCreator, 70),
            (MarketingSpecialist, 20),
            (SalesRep, 10),
        ],
    }
}
