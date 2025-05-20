use crate::dashboard_viewer::routes::DashboardData;
use ratatui::prelude::*;
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset};
use shared::Financials;

pub fn render_financials_chart(
    frame: &mut Frame,
    area: Rect,
    client_history_state: &Vec<(u16, Financials)>,
    _dashboard_data: &DashboardData,
) {
    let mut min_y = f64::MAX;
    let mut max_y = f64::MIN;

    let mut points_cash = Vec::new();
    let mut points_income = Vec::new();
    let mut points_expenses = Vec::new();
    let mut points_net_profit = Vec::new();

    for (week, f) in client_history_state {
        let week = *week as f64;

        let values = [
            f.actual_cash as f64,
            f.this_weeks_income as f64,
            f.this_weeks_expenses as f64,
            f.this_weeks_net_profit as f64,
        ];

        for v in values {
            min_y = min_y.min(v);
            max_y = max_y.max(v);
        }

        points_cash.push((week, f.actual_cash as f64));
        points_income.push((week, f.this_weeks_income as f64));
        points_expenses.push((week, f.this_weeks_expenses as f64));
        points_net_profit.push((week, f.this_weeks_net_profit as f64));
    }

    // Expand bounds slightly for padding
    let padding = (max_y - min_y).abs() * 0.1;
    let y_bounds = if min_y < max_y {
        [min_y - padding, max_y + padding]
    } else {
        [min_y - 1.0, max_y + 1.0] // avoid zero-range issues
    };

    let x_bounds = match client_history_state
        .first()
        .zip(client_history_state.last())
    {
        Some(((start, _), (end, _))) => [*start as f64, *end as f64],
        None => [0.0, 10.0],
    };

    let datasets = vec![
        Dataset::default()
            .name("Cash")
            .marker(Marker::Dot)
            .style(Style::default().fg(Color::Green))
            .data(&points_cash),
        Dataset::default()
            .name("Income")
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Blue))
            .data(&points_income),
        Dataset::default()
            .name("Expenses")
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Red))
            .data(&points_expenses),
        Dataset::default()
            .name("Net Profit")
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .data(&points_net_profit),
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Financials").borders(Borders::ALL))
        .x_axis(
            Axis::default()
                .title("Week")
                .style(Style::default().fg(Color::Gray))
                .labels(vec![
                    Span::raw(format!("{:.0}", x_bounds[0])),
                    Span::styled("Recent", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!("{:.0}", x_bounds[1])),
                ])
                .bounds(x_bounds),
        )
        .y_axis(
            Axis::default()
                .title("Amount")
                .style(Style::default().fg(Color::Gray))
                .labels(vec![
                    Span::raw(format!("{:.0}", y_bounds[0])),
                    Span::raw("0"),
                    Span::raw(format!("{:.0}", y_bounds[1])),
                ])
                .bounds(y_bounds),
        );

    frame.render_widget(chart, area);
}

pub fn render_financials_legend(frame: &mut Frame, area: Rect) {
    let line = Line::from(vec![
        Span::styled("● Cash ", Style::default().fg(Color::Green)),
        Span::raw("  "),
        Span::styled("● Income ", Style::default().fg(Color::Blue)),
        Span::raw("  "),
        Span::styled("● Expenses ", Style::default().fg(Color::Red)),
        Span::raw("  "),
        Span::styled("● Net Profit", Style::default().fg(Color::Yellow)),
    ]);

    let text = Text::from(vec![line]);
    frame.render_widget(text, area);
}
