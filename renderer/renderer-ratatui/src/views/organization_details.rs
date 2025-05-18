use crate::views::get_age_description;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use renderer_api::ClientGameState;
use shared::Organization;
use uuid::Uuid;

pub fn render_organization_details(
    client_game_state: &ClientGameState,
    frame: &mut Frame,
    left_pane: &Rect,
    right_pane: &Rect,
    org_id: &Uuid,
    selected_index: &usize,
) {
    let organization = client_game_state.organizations.get(org_id).unwrap();
    let employees = client_game_state
        .ordered_employees_of_organization
        .get(org_id);

    draw_employee_list(
        frame,
        left_pane,
        client_game_state,
        organization,
        employees,
        *selected_index,
    );

    if let Some(employees) = employees {
        if let Some(employee_id) = employees.get(*selected_index) {
            let pets = client_game_state
                .ordered_pets_of_entity
                .get(employee_id)
                .map(|v| v.as_slice())
                .unwrap_or(&[]);
            let children = client_game_state
                .ordered_children_of_entity
                .get(employee_id)
                .map(|v| v.as_slice())
                .unwrap_or(&[]);
            draw_employee_details(
                frame,
                right_pane,
                client_game_state.week as i32,
                employee_id,
                client_game_state,
                pets,
                children,
            );
        }
    }
}

pub fn draw_employee_list(
    frame: &mut Frame,
    rect: &Rect,
    client_game_state: &ClientGameState,
    organization: &Organization,
    employee_ids: Option<&Vec<Uuid>>,
    selected_index: usize,
) {
    if let Some(employee_ids) = employee_ids {
        let employees: Vec<_> = employee_ids
            .iter()
            .map(|id| client_game_state.entities.get(id).unwrap())
            .collect();

        let items: Vec<ListItem> = employees
            .iter()
            .map(|e| {
                ListItem::new(format!(
                    "{} (L{})",
                    e.name,
                    e.employment.clone().unwrap().level
                ))
            })
            .collect();

        let mut state = ListState::default();
        state.select(Some(selected_index));

        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!("{} Employees", organization.name))
                    .borders(Borders::ALL),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("➤ ");

        frame.render_stateful_widget(list, *rect, &mut state);
    } else {
        let mut state = ListState::default();
        state.select(Some(0));

        let list = List::new(["(No Employees)".to_string()])
            .block(
                Block::default()
                    .title(format!("{} Employees", organization.name))
                    .borders(Borders::ALL),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("➤ ");

        frame.render_stateful_widget(list, *rect, &mut state);
    }
}

pub fn draw_employee_details(
    frame: &mut Frame,
    rect: &Rect,
    current_week: i32,
    employee_id: &Uuid,
    client_game_state: &ClientGameState,
    pet_ids: &[Uuid],
    child_ids: &[Uuid],
) {
    let employee = client_game_state.entities.get(employee_id).unwrap();

    let pets: Vec<_> = pet_ids
        .iter()
        .map(|id| client_game_state.entities.get(id).unwrap())
        .collect();

    let children: Vec<_> = child_ids
        .iter()
        .map(|id| client_game_state.entities.get(id).unwrap())
        .collect();

    let mut lines: Vec<String>;
    if let Some(employed) = employee.employment.clone() {
        lines = vec![
            format!("Name: {}", employee.name),
            format!(
                "Age: {}",
                get_age_description(
                    current_week.saturating_sub(employee.origin.week_of_birth as i32) as u32
                )
            ),
            format!("Type: {:?}", employee.entity_type),
            format!("Role: {:?}", employed.role),
            format!("Level: {}", employed.level),
            format!("Satisfaction: {}", employed.satisfaction),
            format!("Salary (p/w): ${}", employed.salary),
        ]
    } else {
        lines = vec!["Somehow rendering an unemployed employee as employed".to_string()]
    }

    for child in children {
        lines.push(format!("  └─ Child: {}", child.name));
    }

    for pet in pets {
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
