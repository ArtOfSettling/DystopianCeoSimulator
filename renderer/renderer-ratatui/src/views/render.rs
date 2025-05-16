use crate::routes::{OrganizationTab, Route};
use crate::views::{render_organization_list, render_organization_view};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use shared::GameStateSnapshot;

pub fn render(route: &Route, game_state_snapshot: &GameStateSnapshot, frame: &mut Frame) {
    let outer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let main_area = outer_chunks[0];
    let financial_area = outer_chunks[1];
    let tooltip_area = outer_chunks[2];

    render_financial_summary(frame, financial_area, &game_state_snapshot);
    render_tooltip(frame, tooltip_area, route);

    match route {
        Route::OrganizationList { data } => {
            render_organization_list(game_state_snapshot, frame, &main_area, &data.selected_index)
        }
        Route::OrganizationView { data } => {
            render_organization_view(game_state_snapshot, frame, &main_area, data)
        }
    }
}

fn render_financial_summary(
    frame: &mut Frame,
    rect: Rect,
    game_state_snapshot: &GameStateSnapshot,
) {
    let lines = vec![
        format!("Week: {}", game_state_snapshot.week),
        format!("Cash: ${}", game_state_snapshot.financials.actual_cash),
        format!(
            "Income: ${}",
            game_state_snapshot.financials.this_weeks_income
        ),
        format!(
            "Expenses: ${}",
            game_state_snapshot.financials.this_weeks_expenses
        ),
        format!(
            "Net Profit: ${}",
            game_state_snapshot.financials.this_weeks_net_profit
        ),
    ];

    let block = Block::default().title("Financials").borders(Borders::ALL);

    let paragraph = Paragraph::new(lines.join(" | "))
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, rect);
}

fn render_tooltip(frame: &mut Frame, rect: Rect, route: &Route) {
    let text = match route {
        Route::OrganizationList { .. } => "↑↓ to select | → to view | [q] Quit",
        Route::OrganizationView { data } => match data.tab {
            OrganizationTab::Detail => {
                "← Back to List | ↑↓ Navigate | [Tab] Change Tab | [f] Fire | [r] Raise | [p] Promote | [q] Quit"
            }
            OrganizationTab::Hiring => "← Back to Detail | ↑↓ Navigate | [h] Hire | [q] Quit",
        },
    };

    let tooltip = Paragraph::new(text)
        .style(Style::default().fg(Color::LightBlue))
        .wrap(Wrap { trim: true });

    frame.render_widget(tooltip, rect);
}
