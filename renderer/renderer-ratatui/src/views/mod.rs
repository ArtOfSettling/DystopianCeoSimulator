mod organization_budget;
mod organization_details;
pub mod organization_hiring;
pub mod organization_list;
pub mod organization_view;
pub mod render;

pub use organization_hiring::*;
pub use organization_list::*;
pub use organization_view::*;

fn get_age_description(weeks: u32) -> String {
    let years = weeks / 52;
    let remaining_weeks = weeks % 52;
    format!("{} years, {} weeks", years, remaining_weeks)
}
