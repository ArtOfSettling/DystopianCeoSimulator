use bevy::app::{App, Plugin, Update};
use bevy::prelude::{ResMut, Resource};
use crossterm::event::{self, Event, KeyCode, poll};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, disable_raw_mode, enable_raw_mode};
use input_api::{InputResource, PendingPlayerInputAction, PlayerInputAction};
use std::io::stdout;
use std::time::Duration;
use tracing::info;

#[derive(Default)]
pub struct CrossTermInputPlugins {}
pub struct CrossTermInput {}

#[derive(Resource)]
pub struct TerminalMode {}

impl TerminalMode {
    fn new() -> Self {
        enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen).unwrap();
        Self {}
    }
}

impl Drop for TerminalMode {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
    }
}

impl Plugin for CrossTermInputPlugins {
    fn build(&self, app: &mut App) {
        app.insert_resource(TerminalMode::new())
            .insert_resource(InputResource {})
            .add_systems(Update, update_input);
    }
}

fn update_input(mut pending_player_input_action: ResMut<PendingPlayerInputAction>) {
    if poll(Duration::from_millis(1)).unwrap() {
        if let Ok(Event::Key(key_event)) = event::read() {
            info!("Key pressed: {:?}", key_event);

            let command = match key_event.code {
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                    Some(PlayerInputAction::MenuUp)
                }
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                    Some(PlayerInputAction::MenuDown)
                }
                KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                    Some(PlayerInputAction::MenuBack)
                }
                KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                    Some(PlayerInputAction::MenuSelect)
                }

                KeyCode::Char('<') => Some(PlayerInputAction::MenuDecrement),
                KeyCode::Char('>') => Some(PlayerInputAction::MenuIncrement),
                KeyCode::Enter => Some(PlayerInputAction::MenuCommit),

                KeyCode::Tab => Some(PlayerInputAction::MenuChangeTab),

                KeyCode::Char('h') => Some(PlayerInputAction::SelectEmployeeToHire),
                KeyCode::Char(' ') => Some(PlayerInputAction::DoNothing),
                KeyCode::Char('p') => Some(PlayerInputAction::SelectEmployeeForPromotionToVP),
                KeyCode::Char('l') => Some(PlayerInputAction::LaunchPRCampaign),
                KeyCode::Char('f') => Some(PlayerInputAction::SelectEmployeeToFire),
                KeyCode::Char('r') => Some(PlayerInputAction::SelectEmployeeForRaise),
                KeyCode::Char('q') => Some(PlayerInputAction::Quit),

                KeyCode::Char('1') => Some(PlayerInputAction::CreateNewGame),
                KeyCode::Char('2') => Some(PlayerInputAction::ListGames),

                _ => None,
            };

            if let Some(action) = command {
                info!("Generated command: {:?}", action);
                pending_player_input_action.0 = Some(action);
            } else {
                info!("No command generated for key: {:?}", key_event);
            }
        }
    }
}
