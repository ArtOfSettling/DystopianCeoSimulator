use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Res, ResMut};
use input_api::{PendingPlayerInputAction, PlayerInputAction};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Text;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::Paragraph};
use renderer_api::{Renderer, RendererResource};
use shared::{EmploymentStatus, GameStateSnapshot, PendingPlayerAction, PlayerAction};
use std::io;
use tracing::{debug, error, info};
use uuid::Uuid;

#[derive(Default)]
pub struct RatatuiRendererPlugin {}

pub struct RatatuiRenderer {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    selected_index: usize,
}

impl Plugin for RatatuiRendererPlugin {
    fn build(&self, app: &mut App) {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend).unwrap();
        let renderer = RatatuiRenderer {
            terminal,
            selected_index: 0,
        };

        app.insert_resource(RendererResource::new(Box::new(renderer)))
            .add_systems(Update, render_system);
    }
}

struct DisplayEntry<'a> {
    org_name: &'a str,
    org_vp: &'a Option<Uuid>,
    employee: &'a shared::EmployeeSnapshot,
}

fn flatten_organization_employees<'a>(snapshot: &'a GameStateSnapshot) -> Vec<DisplayEntry<'a>> {
    let mut result = Vec::new();
    for org in &snapshot.organizations {
        for employee in &org.employees {
            result.push(DisplayEntry {
                org_name: &org.name,
                org_vp: &org.vp,
                employee,
            });
        }
    }
    result
}

impl Renderer for RatatuiRenderer {
    fn render(
        &mut self,
        game_state_snapshot: Res<GameStateSnapshot>,
        mut pending_player_input_action: ResMut<PendingPlayerInputAction>,
        mut pending_player_action: ResMut<PendingPlayerAction>,
    ) {
        let flattened = flatten_organization_employees(&game_state_snapshot);
        match &pending_player_input_action.0 {
            Some(input_action) => match input_action {
                PlayerInputAction::MenuUp => {
                    if self.selected_index == 0 {
                        self.selected_index = flattened.len().saturating_sub(1);
                    } else {
                        self.selected_index -= 1;
                    }
                }
                PlayerInputAction::MenuDown => {
                    self.selected_index += 1;
                    if self.selected_index >= flattened.len() {
                        self.selected_index = 0;
                    }
                }
                PlayerInputAction::SelectEmployeeToFire => {
                    if let Some(entry) = flattened.get(self.selected_index) {
                        pending_player_action.0 =
                            Some(PlayerAction::FireEmployee(entry.employee.id));
                    }
                }
                PlayerInputAction::SelectEmployeeForRaise => {
                    if let Some(entry) = flattened.get(self.selected_index) {
                        pending_player_action.0 =
                            Some(PlayerAction::GiveRaise(entry.employee.id, 100));
                    }
                }
                PlayerInputAction::LaunchPRCampaign => {
                    pending_player_action.0 = Some(PlayerAction::LaunchPRCampaign)
                }
                PlayerInputAction::DoNothing => {
                    pending_player_action.0 = Some(PlayerAction::DoNothing)
                }
                PlayerInputAction::PromoteToVP => {
                    if let Some(entry) = flattened.get(self.selected_index) {
                        if let Some(org) = game_state_snapshot
                            .organizations
                            .iter()
                            .find(|org| org.employees.iter().any(|emp| emp.id == entry.employee.id))
                        {
                            pending_player_action.0 =
                                Some(PlayerAction::PromoteToVp {
                                    target_id: org.id,
                                    employee_id: entry.employee.id,
                                });
                        }
                    }
                }
                _ => {
                    info!("Unknown action: {:?}", input_action);
                }
            },
            _ => {}
        }
        pending_player_input_action.0 = None;

        match self.terminal.draw(|frame| {
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(frame.area());

            let inner_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(main_chunks[0]);

            let stats = format!(
                "Money: ${:.2}\nReputation: {}\nOperating Expenses: {}",
                game_state_snapshot.money,
                game_state_snapshot.reputation,
                game_state_snapshot
                    .organizations
                    .iter()
                    .flat_map(|org| org
                        .employees
                        .iter()
                        .filter(|emp| emp.employment_status == EmploymentStatus::Active))
                    .fold(0, |sum, val| sum + val.salary)
            );
            let stats_widget =
                Paragraph::new(Text::raw(stats)).style(Style::default().fg(Color::White));
            frame.render_widget(stats_widget, inner_chunks[0]);

            let mut lines = vec![];
            let mut current_org = "";

            for (i, entry) in flattened.iter().enumerate() {
                if entry.org_name != current_org {
                    current_org = entry.org_name;
                    let vp_name = entry.org_vp.and_then(|vp_id| {
                        flattened
                            .iter()
                            .find(|emp| emp.employee.id == vp_id)
                            .map(|emp| emp.employee.name.clone())
                    });
                    lines.push(Line::from(Span::styled(
                        format!("Org: {}, VP: {:?}", current_org, vp_name),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )));
                }

                let emp_line = format!(
                    "  {} - Satisfaction: {}, Status: {:?}, Salary: {}",
                    entry.employee.name,
                    entry.employee.satisfaction,
                    entry.employee.employment_status,
                    entry.employee.salary,
                );

                if i == self.selected_index {
                    lines.push(Line::from(Span::styled(
                        emp_line,
                        Style::default().fg(Color::Black).bg(Color::White),
                    )));
                } else {
                    lines.push(Line::from(Span::styled(
                        emp_line,
                        Style::default().fg(Color::Yellow),
                    )));
                }
            }

            let employee_list = Paragraph::new(lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Employees by Org"),
            );

            frame.render_widget(employee_list, inner_chunks[1]);

            let help_line = Line::from(vec![
                Span::styled(
                    format!("Week: {:?}  ", game_state_snapshot.week),
                    Style::default().fg(Color::Red),
                ),
                Span::styled("[W/↑] Up  ", Style::default().fg(Color::Yellow)),
                Span::styled("[S/↓] Down  ", Style::default().fg(Color::Yellow)),
                Span::styled("[A/←] Left  ", Style::default().fg(Color::Yellow)),
                Span::styled("[D/→] Right  ", Style::default().fg(Color::Yellow)),
                Span::raw(" | "),
                Span::styled("[F] Fire  ", Style::default().fg(Color::Red)),
                Span::styled("[R] Raise  ", Style::default().fg(Color::Green)),
                Span::styled("[L] PR Campaign  ", Style::default().fg(Color::Cyan)),
                Span::styled("[P] Promote to vp  ", Style::default().fg(Color::Gray)),
                Span::styled("[Space] Do Nothing  ", Style::default().fg(Color::White)),
                Span::styled("[Q] Quit", Style::default().fg(Color::DarkGray)),
            ]);

            let help_bar = Paragraph::new(help_line)
                .block(Block::default().borders(Borders::NONE))
                .style(Style::default().fg(Color::Gray));

            frame.render_widget(help_bar, main_chunks[1]);
        }) {
            Ok(_) => {
                debug!("Render done");
            }
            Err(_) => {
                error!("Render Error");
            }
        }
    }
}

fn render_system(
    mut render_resource: ResMut<RendererResource>,
    game_state_snapshot: Res<GameStateSnapshot>,
    pending_player_input_action: ResMut<PendingPlayerInputAction>,
    pending_player_action: ResMut<PendingPlayerAction>,
) {
    render_resource.renderer.render(
        game_state_snapshot,
        pending_player_input_action,
        pending_player_action,
    );
}
