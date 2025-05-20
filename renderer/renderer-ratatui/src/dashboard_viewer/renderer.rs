use crate::dashboard_viewer::input::handle_input;
use crate::dashboard_viewer::navigation::NavigationStack;
use crate::dashboard_viewer::views::render::render;
use bevy::app::AppExit;
use bevy::prelude::{EventWriter, ResMut};
use input_api::PendingPlayerInputAction;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use ratatui::{CompletedFrame, Terminal};
use renderer_api::{ClientGameState, ClientHistoryState, Renderer};
use shared::PendingPlayerAction;
use std::io;
use tracing::{debug, error};

pub struct RatatuiDashboardRenderer {
    pub(crate) terminal: Terminal<CrosstermBackend<io::Stdout>>,
    pub navigation_stack: NavigationStack,
}

impl Drop for RatatuiDashboardRenderer {
    fn drop(&mut self) {
        if let Err(e) = disable_raw_mode() {
            error!("Failed to disable raw mode: {:?}", e);
        }
        if let Err(e) = execute!(io::stdout(), LeaveAlternateScreen) {
            error!("Failed to leave alternate screen: {:?}", e);
        }
    }
}

impl RatatuiDashboardRenderer {
    fn try_draw_frame(
        &mut self,
        client_history_state: &ClientHistoryState,
        pending_player_input_action: &mut ResMut<PendingPlayerInputAction>,
        exit_writer: &mut EventWriter<AppExit>,
    ) -> Result<CompletedFrame, io::Error> {
        self.terminal.draw(|frame| {
            if let Some(action) = pending_player_input_action.0.take() {
                let continue_execution =
                    handle_input(action, &mut self.navigation_stack, client_history_state);
                if !continue_execution {
                    exit_writer.send(AppExit::Success);
                }
            }

            render(&mut self.navigation_stack, frame, client_history_state);
        })
    }
}

impl Renderer for RatatuiDashboardRenderer {
    fn render(
        &mut self,
        _client_game_state: &ClientGameState,
        client_history_state: &ClientHistoryState,
        mut pending_player_input_action: ResMut<PendingPlayerInputAction>,
        mut _pending_player_action: ResMut<PendingPlayerAction>,
        mut exit_writer: EventWriter<AppExit>,
    ) {
        if let Err(e) = self.try_draw_frame(
            client_history_state,
            &mut pending_player_input_action,
            &mut exit_writer,
        ) {
            error!("Render error: {:?}", e);
        } else {
            debug!("Render done");
        }
    }
}
