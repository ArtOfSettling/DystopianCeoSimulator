use crate::dashboard_viewer::routes::DashboardData;
use ratatui::prelude::*;
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset};
use shared::Perception;

pub fn render_perception_chart(
    frame: &mut Frame,
    area: Rect,
    client_history_state: &[(u16, Perception)],
    _dashboard_data: &DashboardData,
) {
    let public_opinion_points: Vec<(f64, f64)> = client_history_state
        .iter()
        .map(|(week, p)| (*week as f64, p.public_opinion as f64))
        .collect();

    let reputation_points: Vec<(f64, f64)> = client_history_state
        .iter()
        .map(|(week, p)| (*week as f64, p.reputation as f64))
        .collect();

    let datasets = vec![
        Dataset::default()
            .name("Public Opinion")
            .marker(Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&public_opinion_points),
        Dataset::default()
            .name("Reputation")
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Magenta))
            .data(&reputation_points),
    ];

    let x_bounds = bounds(&public_opinion_points, &reputation_points, true);
    let y_bounds = bounds(&public_opinion_points, &reputation_points, false);

    let chart = Chart::new(datasets)
        .block(Block::default().title("Perception").borders(Borders::ALL))
        .x_axis(
            Axis::default()
                .title("Week")
                .style(Style::default().fg(Color::Gray))
                .labels(vec![
                    Span::raw(format!("{:.0}", x_bounds[0])),
                    Span::styled("Weeks", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!("{:.0}", x_bounds[1])),
                ])
                .bounds(x_bounds),
        )
        .y_axis(
            Axis::default()
                .title("Score")
                .style(Style::default().fg(Color::Gray))
                .labels(vec![Span::raw("Low"), Span::raw("Mid"), Span::raw("High")])
                .bounds(y_bounds),
        );

    frame.render_widget(chart, area);
}

fn bounds(a: &[(f64, f64)], b: &[(f64, f64)], is_x: bool) -> [f64; 2] {
    let all = a.iter().chain(b.iter());
    let min = all
        .clone()
        .map(|(x, y)| if is_x { *x } else { *y })
        .fold(f64::INFINITY, f64::min);
    let max = all
        .map(|(x, y)| if is_x { *x } else { *y })
        .fold(f64::NEG_INFINITY, f64::max);

    if min == max {
        [min - 1.0, max + 1.0]
    } else {
        [min, max]
    }
}

pub fn render_perception_legend(frame: &mut Frame, area: Rect) {
    let line = Line::from(vec![
        Span::styled("● Public Opinion ", Style::default().fg(Color::Cyan)),
        Span::raw("  "),
        Span::styled("● Reputation", Style::default().fg(Color::Magenta)),
    ]);

    let text = Text::from(vec![line]);
    frame.render_widget(text, area);
}
