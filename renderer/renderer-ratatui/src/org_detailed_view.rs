use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use shared::{ChildSnapshot, OrganizationSnapshot, PetSnapshot};

pub fn draw_detailed_org_view(
    f: &mut Frame,
    area: Rect,
    organization_snapshot: &OrganizationSnapshot,
    pet_snapshots: &Vec<PetSnapshot>,
    child_snapshots: &Vec<ChildSnapshot>,
    selected_index: usize,
) {
    let mut all_items: Vec<ListItem> = Vec::new();
    let mut selectable_items: Vec<ListItem> = Vec::new();

    // Add org name (non-selectable)
    all_items.push(ListItem::new(format!(
        "Org: {}",
        organization_snapshot.name
    )));

    // VP info (selectable if exists)
    if let Some(vp_id) = organization_snapshot.vp {
        if let Some(vp) = organization_snapshot
            .employees
            .iter()
            .find(|e| e.id == vp_id)
        {
            let vp_line = format!(
                "VP: {} (L{}) | Role: {} | Satisfaction: {} | Salary: ${} | Status: {:?}",
                vp.name, vp.level, vp.role, vp.satisfaction, vp.salary, vp.employment_status
            );
            all_items.push(ListItem::new(vp_line.clone()));
            selectable_items.push(ListItem::new(vp_line)); // Add VP as selectable

            // Add children
            for child in child_snapshots
                .iter()
                .filter(|child| vp.children_ids.contains(&child.id))
            {
                let child_line = format!(
                    "      └── Child: {} (Age {})",
                    child.name, "not modelled yet"
                );
                all_items.push(ListItem::new(child_line.clone()));
                selectable_items.push(ListItem::new(child_line));
            }

            // Add pets
            for pet in pet_snapshots
                .iter()
                .filter(|pet| vp.pet_ids.contains(&pet.id))
            {
                let pet_line = format!("      └── Pet: {} ({:?})", pet.name, pet.pet_type);
                all_items.push(ListItem::new(pet_line.clone()));
                selectable_items.push(ListItem::new(pet_line));
            }
        } else {
            let line = "VP: [Unknown]".to_string();
            all_items.push(ListItem::new(line));
        }
    } else {
        all_items.push(ListItem::new("VP: [Vacant]".to_string()));
    }

    // Employees (excluding VP)
    let mut employees: Vec<_> = organization_snapshot
        .employees
        .iter()
        .filter(|e| Some(e.id) != organization_snapshot.vp)
        .collect();

    employees.sort_by_key(|e| -(e.level as i32)); // Descending level

    for emp in employees {
        let emp_line = format!(
            "  └── {} (L{}) | Role: {} | Satisfaction: {} | Salary: ${} | Status: {:?}",
            emp.name, emp.level, emp.role, emp.satisfaction, emp.salary, emp.employment_status
        );
        all_items.push(ListItem::new(emp_line.clone()));
        selectable_items.push(ListItem::new(emp_line));

        // Add children
        for child in child_snapshots
            .iter()
            .filter(|child| emp.children_ids.contains(&child.id))
        {
            let child_line = format!(
                "      └── Child: {} (Age {})",
                child.name, "not modelled yet"
            );
            all_items.push(ListItem::new(child_line.clone()));
            selectable_items.push(ListItem::new(child_line));
        }

        // Add pets
        for pet in pet_snapshots
            .iter()
            .filter(|pet| emp.pet_ids.contains(&pet.id))
        {
            let pet_line = format!("      └── Pet: {} ({:?})", pet.name, pet.pet_type);
            all_items.push(ListItem::new(pet_line.clone()));
            selectable_items.push(ListItem::new(pet_line));
        }
    }

    // Highlight just within the selectable entries
    let mut list_state = ListState::default();

    // Figure out the index of the highlighted item in all_items
    // Offset = 1 if VP exists, otherwise 0
    let highlight_index_in_all_items = if organization_snapshot.vp.is_some() {
        selected_index + 1 // org name = 0, VP = 1
    } else {
        selected_index + 2 // org name = 0, VP line missing, skip "Employees:" at 1
    };

    if highlight_index_in_all_items < all_items.len() {
        list_state.select(Some(highlight_index_in_all_items));
    }

    let org_list = List::new(all_items)
        .block(
            Block::default()
                .title("Detailed View")
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("➤ ");

    f.render_stateful_widget(org_list, area, &mut list_state);
}
