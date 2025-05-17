use crate::views::get_age_description;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Wrap};
use ratatui::{Frame, widgets::Paragraph};
use shared::{GameStateSnapshot, OrganizationSnapshot, UnemployedSnapshot};
use uuid::Uuid;

pub fn render_hiring(
    game_state_snapshot: &GameStateSnapshot,
    frame: &mut Frame,
    left_pane: &Rect,
    right_pane: &Rect,
    org_id: &Uuid,
    selected_index: &usize,
) {
    let org = game_state_snapshot
        .organizations
        .iter()
        .find(|o| o.id == *org_id);
    draw_unemployed_list(
        frame,
        left_pane,
        &game_state_snapshot.unemployed,
        *selected_index,
    );
    if let Some(person) = game_state_snapshot.unemployed.get(*selected_index) {
        draw_candidate_details(
            frame,
            right_pane,
            game_state_snapshot.week as i32,
            person,
            org.unwrap(),
        );
    }
}

pub fn draw_unemployed_list(
    frame: &mut Frame,
    rect: &Rect,
    unemployed_snapshot: &[UnemployedSnapshot],
    selected_index: usize,
) {
    let items: Vec<ListItem> = unemployed_snapshot
        .iter()
        .map(|p| {
            ListItem::new(
                (match p {
                    UnemployedSnapshot::UnemployedAnimalSnapshot(animal) => animal.name.clone(),
                    UnemployedSnapshot::UnemployedHumanSnapshot(human) => human.name.clone(),
                })
                .to_string(),
            )
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(selected_index.min(items.len().saturating_sub(1))));

    let list = List::new(items)
        .block(
            Block::default()
                .title("Unemployed Candidates")
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

pub fn draw_candidate_details(
    frame: &mut Frame,
    rect: &Rect,
    current_week: i32,
    unemployed_snapshot: &UnemployedSnapshot,
    org_snapshot: &OrganizationSnapshot,
) {
    let mut lines = Vec::new();

    match unemployed_snapshot {
        UnemployedSnapshot::UnemployedAnimalSnapshot(animal) => {
            lines.push(format!("Name: {}", animal.name));
            lines.push(format!("Type: {:?}", animal.entity_type));
            lines.push(format!(
                "Age: {}",
                get_age_description(current_week.saturating_sub(animal.week_of_birth) as u32)
            ));
            lines.push(format!("ID: {}", animal.id));
            lines.push("—".into());
            lines.push(format!("Considering Org: {}", org_snapshot.name));
        }
        UnemployedSnapshot::UnemployedHumanSnapshot(human) => {
            lines.push(format!("Name: {}", human.name));
            lines.push(format!("ID: {}", human.id));
            lines.push(format!(
                "Age: {}",
                get_age_description(current_week.saturating_sub(human.week_of_birth) as u32)
            ));
            lines.push("—".into());
            lines.push(format!("Considering Org: {}", org_snapshot.name));
        }
    }

    let block = Block::default()
        .title("Candidate Details")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(lines.join("\n"))
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, *rect);
}
