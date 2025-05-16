use crate::navigation::NavigationStack;
use crate::renderer::RatatuiRenderer;
use crate::system::render_system;
use bevy::app::{App, Plugin, Update};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use renderer_api::RendererResource;
use std::io;

#[derive(Default)]
pub struct RatatuiRendererPlugin {}

impl Plugin for RatatuiRendererPlugin {
    fn build(&self, app: &mut App) {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend).unwrap();
        let renderer = RatatuiRenderer {
            terminal,
            navigation_stack: NavigationStack::new(),
        };

        app.insert_resource(RendererResource::new(Box::new(renderer)))
            .add_systems(Update, render_system);
    }
}
