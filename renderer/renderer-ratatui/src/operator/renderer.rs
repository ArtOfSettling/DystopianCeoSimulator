use crate::operator::input::handle_input;
use crate::operator::navigation::NavigationStack;
use crate::operator::views::render::render;
use bevy::prelude::{Res, ResMut};
use input_api::{PendingPlayerInputAction, PlayerInputAction};
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use ratatui::layout::Alignment;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{CompletedFrame, Terminal};
use renderer_api::{ClientGameState, ClientHistoryState, Renderer};
use shared::ClientMessage::{CreateGame, ListGames};
use shared::{ConnectionState, ConnectionStateResource, PendingClientMessage, PendingPlayerAction};
use std::io;
use tracing::{debug, error};

pub struct RatatuiOperatorRenderer {
    pub(crate) terminal: Terminal<CrosstermBackend<io::Stdout>>,
    pub navigation_stack: NavigationStack,
}

impl Drop for RatatuiOperatorRenderer {
    fn drop(&mut self) {
        if let Err(e) = disable_raw_mode() {
            error!("Failed to disable raw mode: {:?}", e);
        }
        if let Err(e) = execute!(io::stdout(), LeaveAlternateScreen) {
            error!("Failed to leave alternate screen: {:?}", e);
        }
    }
}

impl RatatuiOperatorRenderer {
    fn try_draw_frame(
        &mut self,
        connection_state: &ConnectionState,
        client_game_state: &ClientGameState,
        client_history_state: &ClientHistoryState,
        pending_player_input_action: &mut ResMut<PendingPlayerInputAction>,
        pending_player_action: &mut ResMut<PendingPlayerAction>,
    ) -> Result<CompletedFrame, io::Error> {
        self.terminal.draw(|frame| {
            let size = frame.area();

            match connection_state {
                ConnectionState::Connecting => {
                    let paragraph = Paragraph::new(Line::from("🟡 Connecting..."))
                        .alignment(Alignment::Center)
                        .block(Block::default().borders(Borders::ALL).title("Status"));
                    frame.render_widget(paragraph, size);
                }
                ConnectionState::Reconnecting {
                    attempts,
                    next_attempt_in,
                } => {
                    let paragraph = Paragraph::new(Line::from(format!(
                        "🟡 Reconnecting (attempt {}) in {}s...",
                        attempts, next_attempt_in
                    )))
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL).title("Status"));
                    frame.render_widget(paragraph, size);
                }
                ConnectionState::Disconnected => {
                    let paragraph = Paragraph::new(Line::from("🔴 Disconnected"))
                        .alignment(Alignment::Center)
                        .block(Block::default().borders(Borders::ALL).title("Status"));
                    frame.render_widget(paragraph, size);
                }
                ConnectionState::Error(e) => {
                    let paragraph = Paragraph::new(Line::from(format!("🔴 Error: {}", e)))
                        .alignment(Alignment::Center)
                        .block(Block::default().borders(Borders::ALL).title("Status"));
                    frame.render_widget(paragraph, size);
                }
                ConnectionState::Connected => {
                    if let Some(action) = pending_player_input_action.0.take() {
                        handle_input(
                            action,
                            &mut self.navigation_stack,
                            client_game_state,
                            pending_player_action,
                        );
                    }

                    let current = self.navigation_stack.current();
                    render(current, client_game_state, client_history_state, frame);
                }
                ConnectionState::Rejected(reason) => {
                    let paragraph =
                        Paragraph::new(Line::from(format!("🔴 Server Rejected: {}", reason)))
                            .alignment(Alignment::Center)
                            .block(Block::default().borders(Borders::ALL).title("Status"));
                    frame.render_widget(paragraph, size);
                }
            }
        })
    }
}

impl Renderer for RatatuiOperatorRenderer {
    fn render(
        &mut self,
        client_game_state: &ClientGameState,
        client_history_state: &ClientHistoryState,
        mut pending_client_message: ResMut<PendingClientMessage>,
        mut pending_player_input_action: ResMut<PendingPlayerInputAction>,
        mut pending_player_action: ResMut<PendingPlayerAction>,
        connection_state_resource: Res<ConnectionStateResource>,
    ) {
        if let Some(PlayerInputAction::CreateNewGame) = &pending_player_input_action.0 {
            let game_name: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .map(char::from)
                .collect();

            pending_client_message.0 = Some(CreateGame { game_name });
        }

        if let Some(PlayerInputAction::ListGames) = &pending_player_input_action.0 {
            pending_client_message.0 = Some(ListGames);
        }

        if let Err(e) = self.try_draw_frame(
            &connection_state_resource.connection_state,
            client_game_state,
            client_history_state,
            &mut pending_player_input_action,
            &mut pending_player_action,
        ) {
            error!("Render error: {:?}", e);
        } else {
            debug!("Render done");
        }
    }
}
