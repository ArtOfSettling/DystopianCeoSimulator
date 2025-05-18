use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use shared::GameStateSnapshot;
use uuid::Uuid;

#[allow(clippy::too_many_arguments)]
pub fn render_organization_budget(
    game_state_snapshot: &GameStateSnapshot,
    frame: &mut Frame,
    left_pane: &Rect,
    right_pane: &Rect,
    org_id: &Uuid,
    selected_index: &usize,
    marketing: &u32,
    training: &u32,
    rnd: &u32,
) {
    let items = ["Marketing", "R&D", "Training"]
        .iter()
        .map(|label| ListItem::new(*label))
        .collect::<Vec<_>>();

    let mut state = ListState::default();
    state.select(Some(*selected_index));

    let list = List::new(items)
        .block(
            Block::default()
                .title(format!(
                    "{} Budget Editor",
                    game_state_snapshot
                        .organizations
                        .iter()
                        .find(|org| org.id == *org_id)
                        .unwrap()
                        .name
                ))
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("➤ ");

    frame.render_stateful_widget(list, *left_pane, &mut state);

    let lines = [
        format!("Marketing Budget (p/w): ${}", marketing),
        format!("R&D Budget (p/w):       ${}", rnd),
        format!("Training Budget (p/w):  ${}", training),
    ];

    let block = Block::default()
        .title("Budget Details")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(lines.join("\n"))
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, *right_pane);
}
