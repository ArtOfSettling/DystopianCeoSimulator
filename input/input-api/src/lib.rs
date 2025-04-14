use bevy::prelude::Resource;
use shared::PlayerAction;

#[derive(Resource)]
pub struct InputResource {
    pub input_handler: Box<dyn InputHandler + Send + Sync>,
}

impl InputResource {
    pub fn new(input_handler: Box<dyn InputHandler + Send + Sync>) -> InputResource {
        Self { input_handler }
    }
}

pub trait InputHandler {
    fn get_player_action(&self) -> Option<PlayerAction>;
}
