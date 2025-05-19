use crate::routes::{OrganizationTab, Route};
use crate::views::{render_organization_list, render_organization_view};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use renderer_api::{ClientGameState, ClientHistoryState};

pub fn render(
    route: &Route,
    client_game_state: &ClientGameState,
    client_history_state: &ClientHistoryState,
    frame: &mut Frame,
) {
    if client_game_state.players.is_empty() {
        return;
    }

    let outer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let main_area = outer_chunks[0];
    let vitals_area = outer_chunks[1];
    let financial_area = outer_chunks[2];
    let tooltip_area = outer_chunks[3];

    render_vitals_summary(frame, vitals_area, client_game_state);
    render_financial_summary(frame, financial_area, client_game_state);
    render_tooltip(frame, tooltip_area, route);

    let (company_id, _company) = client_game_state.companies.iter().next().unwrap();

    match route {
        Route::OrganizationList { data } => render_organization_list(
            client_game_state,
            client_history_state,
            frame,
            &main_area,
            company_id,
            &data.selected_index,
        ),
        Route::OrganizationView { data } => {
            render_organization_view(client_game_state, frame, &main_area, data)
        }
    }
}

fn render_vitals_summary(frame: &mut Frame, rect: Rect, client_game_state: &ClientGameState) {
    let player = client_game_state.players.first().unwrap();

    let lines = [
        format!("Week: {}", client_game_state.week),
        format!("CEO Public Opinion: {}", player.perception.public_opinion),
        format!("CEO Reputation: {}", player.perception.reputation),
    ];

    let block = Block::default().title("Vitals").borders(Borders::ALL);

    let paragraph = Paragraph::new(lines.join(" | "))
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, rect);
}

fn render_financial_summary(frame: &mut Frame, rect: Rect, client_game_state: &ClientGameState) {
    client_game_state
        .companies
        .iter()
        .for_each(|(_uuid, company)| {
            let lines = [
                format!("Cash: ${}", company.financials.actual_cash),
                format!("Income: ${}", company.financials.this_weeks_income),
                format!("Expenses: ${}", company.financials.this_weeks_expenses),
                format!("Net Profit: ${}", company.financials.this_weeks_net_profit),
            ];

            let block = Block::default().title("Financials").borders(Borders::ALL);

            let paragraph = Paragraph::new(lines.join(" | "))
                .block(block)
                .wrap(Wrap { trim: true });

            frame.render_widget(paragraph, rect);
        });
}

fn render_tooltip(frame: &mut Frame, rect: Rect, route: &Route) {
    let text = match route {
        Route::OrganizationList { .. } => {
            "↑↓ to select | → to view | [Space] Wait a week | [q] Quit"
        }
        Route::OrganizationView { data } => match data.tab {
            OrganizationTab::Detail => {
                "← Back to List | ↑↓ Navigate | [Tab] Change Tab | [f] Fire | [r] Raise | [p] Promote | [q] Quit"
            }
            OrganizationTab::Budget => {
                "← Back to Detail | ↑↓ Navigate | < > Adjust | [Enter] Confirm"
            }
            OrganizationTab::Hiring => "← Back to Detail | ↑↓ Navigate | [h] Hire | [q] Quit",
        },
    };

    let tooltip = Paragraph::new(text)
        .style(Style::default().fg(Color::LightBlue))
        .wrap(Wrap { trim: true });

    frame.render_widget(tooltip, rect);
}
