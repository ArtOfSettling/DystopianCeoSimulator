use rand::prelude::{SliceRandom, StdRng};
use shared::CompanyType;
use std::collections::VecDeque;

pub fn build_all_company_name_pool(
    rng: &mut StdRng,
    company_type: &CompanyType,
) -> VecDeque<String> {
    match company_type {
        CompanyType::ECommerce => {
            let prefixes = vec![
                "Quick", "Bright", "Prime", "Swift", "Urban", "Next", "Fresh", "Zoom",
            ];
            let suffixes = vec![
                "Cart", "Bazaar", "Market", "Store", "Shop", "Depot", "Outlet", "Mall",
            ];

            let mut names = Vec::new();
            for prefix in &prefixes {
                for suffix in &suffixes {
                    names.push(format!("{} {}", prefix, suffix));
                }
            }
            names.shuffle(rng);
            VecDeque::from(names)
        }
    }
}
