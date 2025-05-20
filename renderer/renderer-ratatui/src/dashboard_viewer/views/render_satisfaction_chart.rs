use crate::dashboard_viewer::routes::DashboardData;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Sparkline};

pub fn render_satisfaction_chart(
    frame: &mut Frame,
    area: Rect,
    client_history_state: &[(u16, u16)],
    _dashboard_data: &DashboardData,
) {
    let data: Vec<u64> = client_history_state
        .iter()
        .map(|(_, score)| *score as u64)
        .collect();

    let sparkline = Sparkline::default()
        .block(Block::default().title("Satisfaction").borders(Borders::ALL))
        .data(&data)
        .style(Style::default().fg(Color::Green))
        .max(*data.iter().max().unwrap_or(&1));

    frame.render_widget(sparkline, area);
}
