use crate::dashboard_viewer::navigation::NavigationStack;
use crate::dashboard_viewer::routes::{DashboardData, EntityKind, Route, SelectedEntityKindData};
use crate::dashboard_viewer::views::render_entities::render_entities;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use renderer_api::ClientHistoryState;
use std::collections::HashMap;

pub fn render(
    navigation_stack: &mut NavigationStack,
    frame: &mut Frame,
    client_history_state: &ClientHistoryState,
) {
    let outer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)])
        .split(frame.area());

    let main_area = outer_chunks[0];
    let current_mut = navigation_stack.current_mut();

    match current_mut {
        Route::Dashboard { data } => {
            populate_data(data, client_history_state);
            render_entities(frame, main_area, client_history_state, data);
        }
    }
}

fn populate_data(dashboard_data: &mut DashboardData, client_history_state: &ClientHistoryState) {
    let mut selected_entity_kind_data = HashMap::new();
    selected_entity_kind_data.insert(
        EntityKind::Player,
        SelectedEntityKindData {
            selected_index: dashboard_data
                .selected_entity_kind_data
                .get(&EntityKind::Player)
                .map_or(0, |a| a.selected_index),
            entity_count: client_history_state.history_state.players.len(),
        },
    );

    selected_entity_kind_data.insert(
        EntityKind::Organization,
        SelectedEntityKindData {
            selected_index: dashboard_data
                .selected_entity_kind_data
                .get(&EntityKind::Organization)
                .map_or(0, |a| a.selected_index),
            entity_count: client_history_state.history_state.organizations.len(),
        },
    );

    selected_entity_kind_data.insert(
        EntityKind::Company,
        SelectedEntityKindData {
            selected_index: dashboard_data
                .selected_entity_kind_data
                .get(&EntityKind::Company)
                .map_or(0, |a| a.selected_index),
            entity_count: client_history_state.history_state.companies.len(),
        },
    );

    dashboard_data.selected_entity_kind_data = selected_entity_kind_data;
}
