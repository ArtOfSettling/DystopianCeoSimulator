use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Wrap};
use ratatui::{Frame, widgets::Paragraph};
use shared::{GameStateSnapshot, OrganizationSnapshot};

pub fn render_organization_list(
    game_state_snapshot: &GameStateSnapshot,
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

    let organization = game_state_snapshot.organizations.get(*selected_index);
    render_organizations(
        frame,
        &left_pane,
        &game_state_snapshot.organizations,
        *selected_index,
    );
    if let Some(organization) = organization {
        render_organization_summary(frame, &right_pane, organization);
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
    let lines = [format!("Org Name: {}", org_snapshot.name),
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
        format!("Employees: {}", org_snapshot.employees.len()),
        format!("Cash: ${}", org_snapshot.financials.actual_cash),
        format!("Income: ${}", org_snapshot.financials.this_weeks_income),
        format!("Expenses: ${}", org_snapshot.financials.this_weeks_expenses),
        format!(
            "Net Profit: ${}",
            org_snapshot.financials.this_weeks_net_profit
        )];

    let block = Block::default()
        .title("Organization Summary")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(lines.join("\n"))
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, *rect);
}
