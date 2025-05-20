use crate::dashboard_viewer::routes::{DashboardData, EntityKind};
use crate::dashboard_viewer::views::render_financials_chart::{
    render_financials_chart, render_financials_legend,
};
use crate::dashboard_viewer::views::render_perception_chart::{
    render_perception_chart, render_perception_legend,
};
use crate::dashboard_viewer::views::render_satisfaction_chart::render_satisfaction_chart;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph};
use renderer_api::ClientHistoryState;
use shared::{Financials, Perception};

pub fn render_entity_details(
    frame: &mut Frame,
    area: Rect,
    client_history_state: &ClientHistoryState,
    dashboard_data: &DashboardData,
) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Length(15),
            Constraint::Min(8),
        ])
        .split(area);

    render_header(frame, main_chunks[0], client_history_state, dashboard_data);
    render_charts(frame, main_chunks[1], client_history_state, dashboard_data);
    render_history_table(frame, main_chunks[2]);
}

pub fn render_header(
    frame: &mut Frame,
    area: Rect,
    client_history_state: &ClientHistoryState,
    dashboard_data: &DashboardData,
) {
    let kind = &dashboard_data.entity_kind;
    let selected_index = dashboard_data
        .selected_entity_kind_data
        .get(kind)
        .map(|selected_entity_kind_data| selected_entity_kind_data.selected_index)
        .unwrap_or(0);

    fn short_id(id: &uuid::Uuid) -> String {
        id.to_string()[..8].to_string()
    }

    let mut spans = vec![Span::raw("No entity selected\n")];

    match kind {
        EntityKind::Player => {
            let players = &client_history_state.history_state.players;
            if let Some((id, player)) = players.iter().nth(selected_index) {
                if let Some(history) = player.recent_history.back() {
                    spans = vec![
                        Span::raw(format!("Player ID: {}\n", short_id(id))),
                        Span::raw(format!("Cash: ${}\n", history.financials.actual_cash)),
                        Span::raw(format!(
                            "Income: ${}\n",
                            history.financials.this_weeks_income
                        )),
                        Span::raw(format!(
                            "Expenses: ${}\n",
                            history.financials.this_weeks_expenses
                        )),
                        Span::raw(format!(
                            "Net Profit: ${}\n",
                            history.financials.this_weeks_net_profit
                        )),
                        Span::raw(format!(
                            "Public Opinion: {}\n",
                            history.perception.public_opinion
                        )),
                        Span::raw(format!("Reputation: {}\n", history.perception.reputation)),
                        Span::raw(format!("History Points: {}", player.recent_history.len())),
                    ];
                }
            }
        }
        EntityKind::Organization => {
            let orgs = &client_history_state.history_state.organizations;
            if let Some((id, org)) = orgs.iter().nth(selected_index) {
                if let Some(history) = org.recent_history.back() {
                    spans = vec![
                        Span::raw(format!("Organization ID: {}\n", short_id(id))),
                        Span::raw(format!("Cash: ${}\n", history.financials.actual_cash)),
                        Span::raw(format!(
                            "Income: ${}\n",
                            history.financials.this_weeks_income
                        )),
                        Span::raw(format!(
                            "Expenses: ${}\n",
                            history.financials.this_weeks_expenses
                        )),
                        Span::raw(format!(
                            "Net Profit: ${}\n",
                            history.financials.this_weeks_net_profit
                        )),
                        Span::raw(format!(
                            "Public Opinion: {}\n",
                            history.perception.public_opinion
                        )),
                        Span::raw(format!("Reputation: {}\n", history.perception.reputation)),
                        Span::raw(format!("History Points: {}", org.recent_history.len())),
                    ];
                }
            }
        }
        EntityKind::Company => {
            let companies = &client_history_state.history_state.companies;
            if let Some((id, comp)) = companies.iter().nth(selected_index) {
                if let Some(history) = comp.recent_history.back() {
                    spans = vec![
                        Span::raw(format!("Company ID: {}\n", short_id(id))),
                        Span::raw(format!("Cash: ${}\n", history.financials.actual_cash)),
                        Span::raw(format!(
                            "Income: ${}\n",
                            history.financials.this_weeks_income
                        )),
                        Span::raw(format!(
                            "Expenses: ${}\n",
                            history.financials.this_weeks_expenses
                        )),
                        Span::raw(format!(
                            "Net Profit: ${}\n",
                            history.financials.this_weeks_net_profit
                        )),
                        Span::raw(format!(
                            "Public Opinion: {}\n",
                            history.perception.public_opinion
                        )),
                        Span::raw(format!("Reputation: {}\n", history.perception.reputation)),
                        Span::raw(format!("History Points: {}", comp.recent_history.len())),
                    ];
                }
            }
        }
    }

    // Construct Text from Spans
    let lines: Vec<Line> = spans
        .into_iter()
        .map(|span| Line::from(vec![span]))
        .collect();
    let text = Text::from(lines);

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Entity Summary"),
        )
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

pub fn render_charts(
    frame: &mut Frame,
    area: Rect,
    client_history_state: &ClientHistoryState,
    dashboard_data: &DashboardData,
) {
    if client_history_state.player_order.is_empty() {
        return;
    }

    let chart_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);

    let entity_kind = dashboard_data.entity_kind.clone();
    let selected_index = dashboard_data
        .selected_entity_kind_data
        .get(&entity_kind)
        .map_or(0, |e| e.selected_index);

    let entry = match entity_kind {
        EntityKind::Player => client_history_state
            .history_state
            .players
            .get(&client_history_state.player_order[selected_index])
            .map(|player| &player.recent_history),
        EntityKind::Company => client_history_state
            .history_state
            .companies
            .get(&client_history_state.company_order[selected_index])
            .map(|company| &company.recent_history),
        EntityKind::Organization => client_history_state
            .history_state
            .organizations
            .get(&client_history_state.organization_order[selected_index])
            .map(|organization| &organization.recent_history),
    };

    if let Some(entry) = entry {
        let financial_data: Vec<(u16, Financials)> = entry
            .iter()
            .map(|entry| (entry.week, entry.financials.clone()))
            .collect();

        let financials_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(2)])
            .split(chart_chunks[0]);
        render_financials_chart(frame, financials_chunks[0], &financial_data, dashboard_data);
        render_financials_legend(frame, financials_chunks[1]);

        let perception_data: Vec<(u16, Perception)> = entry
            .iter()
            .map(|entry| (entry.week, entry.perception.clone()))
            .collect();

        let perception_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(2)])
            .split(chart_chunks[1]);
        render_perception_chart(
            frame,
            perception_chunks[0],
            &perception_data,
            dashboard_data,
        );
        render_perception_legend(frame, perception_chunks[1]);

        let satisfaction_data: Vec<(u16, u16)> = entry
            .iter()
            .map(|entry| (entry.week, entry.avg_employee_satisfaction))
            .collect();
        render_satisfaction_chart(frame, chart_chunks[2], &satisfaction_data, dashboard_data);
    }
}

pub fn render_history_table(frame: &mut Frame, area: Rect) {
    let history_block = Block::default()
        .title("Recent History")
        .borders(Borders::ALL);
    frame.render_widget(history_block, area);
}
