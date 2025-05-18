use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Sparkline, Wrap};
use ratatui::{Frame, widgets::Paragraph};
use shared::{
    GameStateSnapshot, HistoryStateSnapshot, OrgHistoryPoint, OrgHistorySnapshot, OrgInitiative,
    OrganizationSnapshot,
};

pub fn render_organization_list(
    game_state_snapshot: &GameStateSnapshot,
    history_state_snapshot: &HistoryStateSnapshot,
    frame: &mut Frame,
    main_area: &Rect,
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

    let organization = game_state_snapshot.organizations.get(*selected_index);
    render_organizations(
        frame,
        &left_pane,
        &game_state_snapshot.organizations,
        *selected_index,
    );
    if let Some(organization) = organization {
        let id = organization.id;
        let history = history_state_snapshot
            .organizations
            .iter()
            .find(|organization| organization.org_id == id);
        render_organization_summary(frame, &top_pane, organization);
        if let Some(history) = history {
            render_history_graphs(frame, &bottom_pane, history);
        }
    }
}

pub fn render_organizations(
    frame: &mut Frame,
    rect: &Rect,
    org_snapshots: &[OrganizationSnapshot],
    selected_index: usize,
) {
    use ratatui::widgets::{List, ListItem, ListState};

    let items: Vec<ListItem> = org_snapshots
        .iter()
        .map(|org| ListItem::new(format!("Org: {}", org.name)))
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
    org_snapshot: &OrganizationSnapshot,
) {
    let mut lines = vec![
        format!("Org Name: {}", org_snapshot.name),
        format!(
            "VP: {}",
            org_snapshot.vp.map_or("Vacant".to_string(), |id| {
                org_snapshot
                    .employees
                    .iter()
                    .find(|e| e.id == id)
                    .map_or("Unknown".to_string(), |e| e.name.clone())
            })
        ),
        "Budget:".to_string(),
        format!("• Marketing {} (p/w)", org_snapshot.budget.marketing),
        format!("• Training {} (p/w)", org_snapshot.budget.training),
        format!("• R&D {} (p/w)", org_snapshot.budget.rnd),
        format!("Reputation: {}", org_snapshot.reputation),
        format!("Public Opinion: {}", org_snapshot.public_opinion),
        format!("Employees: {}", org_snapshot.employees.len()),
        format!("Cash: ${}", org_snapshot.financials.actual_cash),
        format!("Income: ${}", org_snapshot.financials.this_weeks_income),
        format!("Expenses: ${}", org_snapshot.financials.this_weeks_expenses),
        format!(
            "Net Profit: ${}",
            org_snapshot.financials.this_weeks_net_profit
        ),
    ];

    if !org_snapshot.initiatives.is_empty() {
        lines.push("Active Initiatives:".to_string());
        for initiative in &org_snapshot.initiatives {
            let description = match initiative {
                OrgInitiative::Marketing { weeks_remaining } => {
                    format!("• Marketing ({} weeks left)", weeks_remaining)
                }
                OrgInitiative::Training { weeks_remaining } => {
                    format!("• Training ({} weeks left)", weeks_remaining)
                }
                OrgInitiative::RnD { weeks_remaining } => {
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
    org_history_snapshot: &OrgHistorySnapshot,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
        ])
        .split(*area);

    frame.render_widget(
        Paragraph::new(Span::raw(format!("Org: {}", org_history_snapshot.name))),
        chunks[0],
    );

    let history = |f: fn(&OrgHistoryPoint) -> i32| -> Vec<u64> {
        org_history_snapshot
            .recent_history
            .iter()
            .map(f)
            .map(|v| v.max(0) as u64)
            .collect()
    };

    frame.render_widget(
        Sparkline::default()
            .block(Block::default().title("Net Profit"))
            .data(&history(|h| h.net_profit))
            .style(Style::default().fg(Color::Green)),
        chunks[1],
    );

    frame.render_widget(
        Sparkline::default()
            .block(Block::default().title("Cash"))
            .data(&history(|h| h.cash))
            .style(Style::default().fg(Color::Yellow)),
        chunks[2],
    );

    frame.render_widget(
        Sparkline::default()
            .block(Block::default().title("Public Opinion"))
            .data(&history(|h| h.public_opinion))
            .style(Style::default().fg(Color::Cyan)),
        chunks[3],
    );

    frame.render_widget(
        Sparkline::default()
            .block(Block::default().title("Reputation"))
            .data(&history(|h| h.reputation))
            .style(Style::default().fg(Color::Magenta)),
        chunks[4],
    );

    frame.render_widget(
        Sparkline::default()
            .block(Block::default().title("Satisfaction"))
            .data(&history(|h| h.avg_employee_satisfaction))
            .style(Style::default().fg(Color::Blue)),
        chunks[5],
    );
}
