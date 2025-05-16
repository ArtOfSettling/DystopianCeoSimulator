use crate::views::get_age_description;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use shared::{
    AnimalSnapshot, EmployeeSnapshot, GameStateSnapshot, HumanSnapshot, OrganizationSnapshot,
};
use uuid::Uuid;

pub fn render_organization_details(
    game_state_snapshot: &GameStateSnapshot,
    frame: &mut Frame,
    left_pane: &Rect,
    right_pane: &Rect,
    org_id: &Uuid,
    selected_index: &usize,
) {
    let org = game_state_snapshot
        .organizations
        .iter()
        .find(|o| o.id == *org_id);
    if let Some(org) = org {
        draw_employee_list(frame, left_pane, org, *selected_index);
        if let Some(emp) = org.employees.get(*selected_index) {
            draw_employee_details(
                frame,
                right_pane,
                game_state_snapshot.week as i32,
                emp,
                &game_state_snapshot.pets,
                &game_state_snapshot.humans,
            );
        }
    }
}

pub fn draw_employee_list(
    frame: &mut Frame,
    rect: &Rect,
    org_snapshot: &OrganizationSnapshot,
    selected_index: usize,
) {
    let items: Vec<ListItem> = org_snapshot
        .employees
        .iter()
        .map(|e| ListItem::new(format!("{} (L{})", e.name, e.level)))
        .collect();

    let mut state = ListState::default();
    state.select(Some(selected_index));

    let list = List::new(items)
        .block(Block::default().title("Employees").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("➤ ");

    frame.render_stateful_widget(list, *rect, &mut state);
}

pub fn draw_employee_details(
    frame: &mut Frame,
    rect: &Rect,
    current_week: i32,
    employee_snapshot: &EmployeeSnapshot,
    pets_snapshot: &Vec<AnimalSnapshot>,
    children_snapshot: &Vec<HumanSnapshot>,
) {
    let mut lines = vec![
        format!("Name: {}", employee_snapshot.name),
        format!(
            "Age: {}",
            get_age_description(current_week.saturating_sub(employee_snapshot.week_of_birth) as u32)
        ),
        format!("Type: {:?}", employee_snapshot.entity_type),
        format!("Role: {:?}", employee_snapshot.role),
        format!("Level: {}", employee_snapshot.level),
        format!("Satisfaction: {}", employee_snapshot.satisfaction),
        format!("Salary: ${}", employee_snapshot.salary),
    ];

    // Children
    for child in children_snapshot
        .iter()
        .filter(|c| employee_snapshot.children_ids.contains(&c.id))
    {
        lines.push(format!("  └─ Child: {}", child.name));
    }

    // Pets
    for pet in pets_snapshot
        .iter()
        .filter(|p| employee_snapshot.pet_ids.contains(&p.id))
    {
        lines.push(format!("  └─ Pet: {} ({:?})", pet.name, pet.entity_type));
    }

    let block = Block::default()
        .title("Employee Details")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(lines.join("\n"))
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, *rect);
}
