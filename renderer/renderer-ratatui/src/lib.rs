use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Query, ResMut};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
};
use renderer_api::{Renderer, RendererResource};
use shared::{Player, Position};
use std::io;
use tracing::{debug, error};

#[derive(Default)]
pub struct RatatuiRendererPlugin {}

pub struct RatatuiRenderer {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Plugin for RatatuiRendererPlugin {
    fn build(&self, app: &mut App) {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend).unwrap();
        let renderer = RatatuiRenderer { terminal };

        app.insert_resource(RendererResource::new(Box::new(renderer)))
            .add_systems(Update, render_system);
    }
}

impl Renderer for RatatuiRenderer {
    fn render(&mut self, player_query: Query<(&Player, &Position)>) {
        match self.terminal.draw(|frame| {
            let connected_player_information = player_query
                .iter()
                .map(|(player, position)| {
                    format!(
                        "player_id: ({:?}) is at ({:?}, {:?})",
                        player.id, position.x, position.y
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");

            let block = Paragraph::new(connected_player_information)
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(block, frame.area());
        }) {
            Ok(_) => {
                debug!("Render done");
            }
            Err(_) => {
                error!("Render Error");
            }
        }
    }
}

fn render_system(
    mut render_resource: ResMut<RendererResource>,
    player_query: Query<(&Player, &Position)>,
) {
    render_resource.renderer.render(player_query);
}
