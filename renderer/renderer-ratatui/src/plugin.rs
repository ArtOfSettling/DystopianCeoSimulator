use crate::dashboard_viewer::renderer::RatatuiDashboardRenderer;
use crate::operator::renderer::RatatuiOperatorRenderer;
use crate::system::render_system;
use bevy::app::{App, Plugin, Update};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use renderer_api::RendererResource;
use shared::OperatorMode;
use std::io;

#[derive(Default)]
pub struct RatatuiRendererPlugin {
    pub operator_mode: OperatorMode,
}

impl Plugin for RatatuiRendererPlugin {
    fn build(&self, app: &mut App) {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend).unwrap();

        match self.operator_mode {
            OperatorMode::Operator => {
                let renderer = RatatuiOperatorRenderer {
                    terminal,
                    navigation_stack: crate::operator::navigation::NavigationStack::new(),
                };

                app.insert_resource(RendererResource::new(Box::new(renderer)))
                    .add_systems(Update, render_system);
            }
            OperatorMode::DashboardViewer => {
                let renderer = RatatuiDashboardRenderer {
                    terminal,
                    navigation_stack: crate::dashboard_viewer::navigation::NavigationStack::new(),
                };

                app.insert_resource(RendererResource::new(Box::new(renderer)))
                    .add_systems(Update, render_system);
            }
        }
    }
}
