use bevy::app::{App, Plugin};
use bevy::prelude::Resource;
use crossterm::event::{self, Event, KeyCode, poll};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use input_api::{InputHandler, InputResource};
use shared::PlayerAction;
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
            .insert_resource(InputResource::new(Box::new(CrossTermInput {})));
    }
}

impl InputHandler for CrossTermInput {
    fn get_player_action(&self) -> Option<PlayerAction> {
        if poll(Duration::from_millis(1)).unwrap() {
            if let Ok(Event::Key(key_event)) = event::read() {
                info!("Key pressed: {:?}", key_event);

                let command = match key_event.code {
                    KeyCode::Char('w') => Some(PlayerAction::MovePlayerLocalUp),
                    KeyCode::Char('d') => Some(PlayerAction::MovePlayerLocalRight),
                    KeyCode::Char('s') => Some(PlayerAction::MovePlayerLocalDown),
                    KeyCode::Char('a') => Some(PlayerAction::MovePlayerLocalLeft),
                    _ => None,
                };

                if let Some(ref cmd) = command {
                    info!("Generated command: {:?}", cmd);
                } else {
                    info!("No command generated for key: {:?}", key_event);
                }

                command
            } else {
                None
            }
        } else {
            None
        }
    }
}
