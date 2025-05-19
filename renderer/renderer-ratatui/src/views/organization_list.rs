use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Sparkline, Wrap};
use ratatui::{Frame, widgets::Paragraph};
use renderer_api::{ClientGameState, ClientHistoryState};
use shared::{HistoryPoint, Initiative, OrganizationHistory};
use uuid::Uuid;

pub fn render_organization_list(
    client_game_state: &ClientGameState,
    client_history_state: &ClientHistoryState,
    frame: &mut Frame,
    main_area: &Rect,
    company_id: &Uuid,
    selected_index: &usize,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(*main_area);

    let left_pane = chunks[0];
    let right_pane = chunks[1];

    let detail_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(12)])
        .split(right_pane);

    let top_pane = detail_area[0];
    let bottom_pane = detail_area[1];

    if let Some(organization_ids) = client_game_state
        .ordered_organizations_of_company
        .get(company_id)
    {
        render_organizations(
            frame,
            &left_pane,
            client_game_state,
            organization_ids,
            *selected_index,
        );

        let organization_id = organization_ids[*selected_index];
        render_organization_summary(frame, &top_pane, client_game_state, &organization_id);
        let history = client_history_state
            .history_state
            .organizations
            .iter()
            .find(|(id, _organization)| **id == organization_id);
        if let Some((_organization_id, organization_history)) = history {
            render_history_graphs(frame, &bottom_pane, organization_history);
        }
    }
}

pub fn render_organizations(
    frame: &mut Frame,
    rect: &Rect,
    client_game_state: &ClientGameState,
    organization_ids: &[Uuid],
    selected_index: usize,
) {
    use ratatui::widgets::{List, ListItem, ListState};

    let organizations: Vec<&_> = organization_ids
        .iter()
        .map(|organization_id| {
            client_game_state
                .organizations
                .get(organization_id)
                .unwrap()
        })
        .collect();

    let items: Vec<ListItem> = organizations
        .iter()
        .map(|organization| {
            ListItem::new(format!(
                "{}: ({:?})",
                organization.name, organization.organization_type
            ))
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(selected_index));

    let list = List::new(items)
        .block(
            Block::default()
                .title("Organizations")
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("➤ ");

    frame.render_stateful_widget(list, *rect, &mut state);
}

pub fn render_organization_summary(
    frame: &mut Frame,
    rect: &Rect,
    client_game_state: &ClientGameState,
    organization_id: &Uuid,
) {
    let organization = client_game_state
        .organizations
        .get(organization_id)
        .unwrap();
    let mut lines = vec![
        format!("Name: {}", organization.name),
        format!("Type: {:?}", organization.organization_type),
        format!(
            "VP: {}",
            organization.vp.map_or("Vacant".to_string(), |id| {
                if let Some(vp) = client_game_state.entities.get(&id) {
                    vp.name.clone()
                } else {
                    "Unknown".to_string()
                }
            })
        ),
        "Budget:".to_string(),
        format!("• Marketing {} (p/w)", organization.budget.marketing),
        format!("• Training {} (p/w)", organization.budget.training),
        format!("• R&D {} (p/w)", organization.budget.rnd),
        format!("Reputation: {}", organization.perception.reputation),
        format!("Public Opinion: {}", organization.perception.public_opinion),
        format!(
            "Employees: {}",
            client_game_state
                .ordered_employees_of_organization
                .get(&organization.id)
                .map(|v| v.len())
                .unwrap_or(0)
        ),
        format!("Cash: ${}", organization.financials.actual_cash),
        format!("Income: ${}", organization.financials.this_weeks_income),
        format!("Expenses: ${}", organization.financials.this_weeks_expenses),
        format!(
            "Net Profit: ${}",
            organization.financials.this_weeks_net_profit
        ),
    ];

    if !organization.initiatives.is_empty() {
        lines.push("Active Initiatives:".to_string());
        for initiative in &organization.initiatives {
            let description = match initiative {
                Initiative::Marketing { weeks_remaining } => {
                    format!("• Marketing ({} weeks left)", weeks_remaining)
                }
                Initiative::Training { weeks_remaining } => {
                    format!("• Training ({} weeks left)", weeks_remaining)
                }
                Initiative::RnD { weeks_remaining } => {
                    format!("• R&D ({} weeks left)", weeks_remaining)
                }
            };
            lines.push(description);
        }
    }

    let block = Block::default()
        .title("Organization Summary")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(lines.join("\n"))
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, *rect);
}

fn render_history_graphs(
    frame: &mut Frame,
    area: &Rect,
    organization_history: &OrganizationHistory,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
        ])
        .split(*area);

    let history = |f: fn(&HistoryPoint) -> i32| -> Vec<u64> {
        organization_history
            .recent_history
            .iter()
            .map(f)
            .map(|v| v.max(0) as u64)
            .collect()
    };

    frame.render_widget(
        Sparkline::default()
            .block(Block::default().title("Net Profit"))
            .data(history(|h| h.financials.this_weeks_net_profit))
            .style(Style::default().fg(Color::Green)),
        chunks[0],
    );

    frame.render_widget(
        Sparkline::default()
            .block(Block::default().title("Cash"))
            .data(history(|h| h.financials.actual_cash))
            .style(Style::default().fg(Color::Yellow)),
        chunks[1],
    );

    frame.render_widget(
        Sparkline::default()
            .block(Block::default().title("Public Opinion"))
            .data(history(|h| h.perception.public_opinion as i32))
            .style(Style::default().fg(Color::Cyan)),
        chunks[2],
    );

    frame.render_widget(
        Sparkline::default()
            .block(Block::default().title("Reputation"))
            .data(history(|h| h.perception.reputation as i32))
            .style(Style::default().fg(Color::Magenta)),
        chunks[3],
    );

    frame.render_widget(
        Sparkline::default()
            .block(Block::default().title("Satisfaction"))
            .data(history(|h| h.avg_employee_satisfaction as i32))
            .style(Style::default().fg(Color::Blue)),
        chunks[4],
    );
}
