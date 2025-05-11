use bevy::prelude::{Query, Resource};
use shared::{InternalEntity, Player, Position};

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
    fn render(&mut self, player_query: Query<(&Player, &InternalEntity, &Position)>);
}
