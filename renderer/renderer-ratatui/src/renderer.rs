use crate::input::handle_input;
use crate::navigation::NavigationStack;
use crate::views::render::render;
use bevy::app::AppExit;
use bevy::prelude::{EventWriter, Res, ResMut};
use input_api::PendingPlayerInputAction;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use ratatui::{CompletedFrame, Terminal};
use renderer_api::Renderer;
use shared::{GameStateSnapshot, PendingPlayerAction};
use std::io;
use tracing::{debug, error};

pub struct RatatuiRenderer {
    pub(crate) terminal: Terminal<CrosstermBackend<io::Stdout>>,
    pub navigation_stack: NavigationStack,
}

impl Drop for RatatuiRenderer {
    fn drop(&mut self) {
        if let Err(e) = disable_raw_mode() {
            error!("Failed to disable raw mode: {:?}", e);
        }
        if let Err(e) = execute!(io::stdout(), LeaveAlternateScreen) {
            error!("Failed to leave alternate screen: {:?}", e);
        }
    }
}

impl RatatuiRenderer {
    fn try_draw_frame(
        &mut self,
        game_state_snapshot: &GameStateSnapshot,
        pending_player_input_action: &mut ResMut<PendingPlayerInputAction>,
        pending_player_action: &mut ResMut<PendingPlayerAction>,
        exit_writer: &mut EventWriter<AppExit>,
    ) -> Result<CompletedFrame, io::Error> {
        self.terminal.draw(|frame| {
            if let Some(action) = pending_player_input_action.0.take() {
                let continue_execution = handle_input(
                    action,
                    &mut self.navigation_stack,
                    game_state_snapshot,
                    pending_player_action,
                );
                if !continue_execution {
                    exit_writer.send(AppExit::Success);
                }
            }

            let current = self.navigation_stack.current();
            render(current, game_state_snapshot, frame);
        })
    }
}

impl Renderer for RatatuiRenderer {
    fn render(
        &mut self,
        game_state_snapshot: Res<GameStateSnapshot>,
        mut pending_player_input_action: ResMut<PendingPlayerInputAction>,
        mut pending_player_action: ResMut<PendingPlayerAction>,
        mut exit_writer: EventWriter<AppExit>,
    ) {
        if let Err(e) = self.try_draw_frame(
            &game_state_snapshot,
            &mut pending_player_input_action,
            &mut pending_player_action,
            &mut exit_writer,
        ) {
            error!("Render error: {:?}", e);
        } else {
            debug!("Render done");
        }
    }
}
