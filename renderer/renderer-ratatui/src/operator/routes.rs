use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrganizationList {
    pub selected_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganizationTab {
    Detail,
    Budget,
    Hiring,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrganizationView {
    pub selected_index: usize,
    pub organization_id: Uuid,
    pub tab: OrganizationTab,
    pub marketing: u16,
    pub rnd: u16,
    pub training: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    OrganizationList { data: OrganizationList },
    OrganizationView { data: OrganizationView },
}
