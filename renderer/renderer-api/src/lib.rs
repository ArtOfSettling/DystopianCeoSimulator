use bevy::app::AppExit;
use bevy::prelude::{EventWriter, Res, ResMut, Resource};
use input_api::PendingPlayerInputAction;
use shared::{GameStateSnapshot, PendingPlayerAction};

#[derive(Resource)]
pub struct RendererResource {
    pub renderer: Box<dyn Renderer + Send + Sync>,
}

impl RendererResource {
    pub fn new(renderer: Box<dyn Renderer + Send + Sync>) -> RendererResource {
        Self { renderer }
    }
}

pub trait Renderer {
    fn render(
        &mut self,
        game_state_snapshot: Res<GameStateSnapshot>,
        pending_player_input_action: ResMut<PendingPlayerInputAction>,
        pending_player_action: ResMut<PendingPlayerAction>,
        exit_writer: EventWriter<AppExit>,
    );
}
