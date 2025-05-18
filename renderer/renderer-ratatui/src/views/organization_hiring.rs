use crate::views::get_age_description;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Wrap};
use ratatui::{widgets::Paragraph, Frame};
use renderer_api::ClientGameState;
use shared::Organization;
use uuid::Uuid;

pub fn render_hiring(
    client_game_state: &ClientGameState,
    frame: &mut Frame,
    left_pane: &Rect,
    right_pane: &Rect,
    organization_id: &Uuid,
    selected_index: &usize,
) {
    let organization = client_game_state
        .organizations
        .get(organization_id)
        .unwrap();

    draw_unemployed_list(
        frame,
        left_pane,
        client_game_state,
        &client_game_state.ordered_unemployed_entities,
        *selected_index,
    );

    if let Some(person) = client_game_state
        .ordered_unemployed_entities
        .get(*selected_index)
    {
        draw_candidate_details(frame, right_pane, client_game_state, person, organization);
    }
}

pub fn draw_unemployed_list(
    frame: &mut Frame,
    rect: &Rect,
    client_game_state: &ClientGameState,
    unemployed_ids: &[Uuid],
    selected_index: usize,
) {
    let unemployed: Vec<_> = unemployed_ids
        .iter()
        .map(|id| client_game_state.entities.get(id).unwrap())
        .collect();

    let items: Vec<ListItem> = unemployed
        .iter()
        .map(|p| ListItem::new(p.name.to_string()))
        .collect();

    let mut state = ListState::default();
    state.select(Some(selected_index.min(items.len().saturating_sub(1))));

    let list = List::new(items)
        .block(
            Block::default()
                .title("Unemployed Candidates")
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

pub fn draw_candidate_details(
    frame: &mut Frame,
    rect: &Rect,
    client_game_state: &ClientGameState,
    unemployed_id: &Uuid,
    organization: &Organization,
) {
    let current_week = client_game_state.week as i32;
    let unemployed = client_game_state.entities.get(unemployed_id).unwrap();
    let mut lines = Vec::new();

    lines.push(format!("Name: {}", unemployed.name));
    lines.push(format!("Type: {:?}", unemployed.entity_type));
    lines.push(format!(
        "Age: {}",
        get_age_description(
            current_week.saturating_sub(unemployed.origin.week_of_birth as i32) as u32
        )
    ));
    lines.push(format!("ID: {}", unemployed.id));
    lines.push("—".into());
    lines.push(format!("Considering Org: {}", organization.name));

    let block = Block::default()
        .title("Candidate Details")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(lines.join("\n"))
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, *rect);
}
