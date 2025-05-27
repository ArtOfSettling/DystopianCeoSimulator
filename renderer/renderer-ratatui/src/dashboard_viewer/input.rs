use crate::dashboard_viewer::navigation::{NavigationAction, NavigationStack};
use crate::dashboard_viewer::routes::{DashboardData, Route};
use input_api::PlayerInputAction;
use renderer_api::ClientHistoryState;

pub fn handle_input(
    player_input_action: PlayerInputAction,
    nav: &mut NavigationStack,
    client_history_state: &ClientHistoryState,
) {
    if let PlayerInputAction::MenuBack = player_input_action {
        NavigationAction::Pop.apply(nav);
        return;
    }

    match nav.current_mut() {
        Route::Dashboard { data } => data.handle_input(player_input_action, client_history_state),
    };
}

pub trait InputHandler {
    fn handle_input(
        &mut self,
        action: PlayerInputAction,
        client_history_state: &ClientHistoryState,
    );
}

impl InputHandler for DashboardData {
    fn handle_input(
        &mut self,
        action: PlayerInputAction,
        _client_history_state: &ClientHistoryState,
    ) {
        match action {
            PlayerInputAction::MenuUp => self.move_selection(
                1,
                self.selected_entity_kind_data
                    .get(&self.entity_kind)
                    .map_or(0, |a| a.entity_count),
            ),
            PlayerInputAction::MenuDown => self.move_selection(
                -1,
                self.selected_entity_kind_data
                    .get(&self.entity_kind)
                    .map_or(0, |a| a.entity_count),
            ),
            PlayerInputAction::MenuChangeTab => self.cycle_kind(),
            _ => {}
        }
    }
}
