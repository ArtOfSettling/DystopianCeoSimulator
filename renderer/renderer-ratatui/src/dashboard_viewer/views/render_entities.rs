use crate::dashboard_viewer::routes::{DashboardData, EntityKind};
use crate::dashboard_viewer::views::render_entity_view::render_entity_details;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use renderer_api::ClientHistoryState;
use shared::HistoryState;
use uuid::Uuid;

pub fn render_entities(
    frame: &mut Frame,
    area: Rect,
    client_history_state: &ClientHistoryState,
    dashboard_data: &DashboardData,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(area);

    let sidebar_area = chunks[0];
    let main_area = chunks[1];

    render_sidebar(frame, sidebar_area, client_history_state, dashboard_data);
    render_entity_details(frame, main_area, client_history_state, dashboard_data);
}

pub fn render_sidebar(
    frame: &mut Frame,
    area: Rect,
    client_history_state: &ClientHistoryState,
    dashboard_data: &DashboardData,
) {
    let items: Vec<ListItem> = entity_labels(
        &client_history_state.history_state,
        dashboard_data.entity_kind.clone(),
    )
    .into_iter()
    .map(|(_, label)| ListItem::new(label))
    .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Organizations")
                .borders(Borders::ALL),
        )
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
        .highlight_symbol("▶ ");

    let mut state = ListState::default();

    let selected_index = dashboard_data.selected_index().unwrap_or(0);
    state.select(Some(selected_index));

    frame.render_stateful_widget(list, area, &mut state);
}

fn entity_labels(history: &HistoryState, kind: EntityKind) -> Vec<(&Uuid, String)> {
    match kind {
        EntityKind::Player => history
            .players
            .keys()
            .map(|id| (id, format!("Player {}", &id.to_string()[..8])))
            .collect(),
        EntityKind::Organization => history
            .organizations
            .keys()
            .map(|id| (id, format!("Org {}", &id.to_string()[..8])))
            .collect(),
        EntityKind::Company => history
            .companies
            .keys()
            .map(|id| (id, format!("Co {}", &id.to_string()[..8])))
            .collect(),
    }
}
