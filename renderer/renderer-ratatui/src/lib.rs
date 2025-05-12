use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Res, ResMut};
use input_api::{PendingPlayerInputAction, PlayerInputAction};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Text;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::Paragraph};
use renderer_api::{Renderer, RendererResource};
use shared::{EmploymentStatus, GameStateSnapshot, PendingPlayerAction, PlayerAction};
use std::io;
use tracing::{debug, error, info};

#[derive(Default)]
pub struct RatatuiRendererPlugin {}

pub struct RatatuiRenderer {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    selected_employee: usize,
}

impl Plugin for RatatuiRendererPlugin {
    fn build(&self, app: &mut App) {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend).unwrap();
        let renderer = RatatuiRenderer {
            terminal,
            selected_employee: 0,
        };

        app.insert_resource(RendererResource::new(Box::new(renderer)))
            .add_systems(Update, render_system);
    }
}

impl Renderer for RatatuiRenderer {
    fn render(
        &mut self,
        game_state_snapshot: Res<GameStateSnapshot>,
        mut pending_player_input_action: ResMut<PendingPlayerInputAction>,
        mut pending_player_action: ResMut<PendingPlayerAction>,
    ) {
        match &pending_player_input_action.0 {
            Some(input_action) => match input_action {
                PlayerInputAction::MenuUp => {
                    if self.selected_employee <= 0 {
                        self.selected_employee = game_state_snapshot.employees.len() - 1;
                    } else {
                        self.selected_employee -= 1;
                    }
                }
                PlayerInputAction::MenuDown => {
                    self.selected_employee += 1;
                    if self.selected_employee > game_state_snapshot.employees.len() - 1 {
                        self.selected_employee = 0;
                    }
                }
                PlayerInputAction::SelectEmployeeToFire => {
                    pending_player_action.0 = Some(PlayerAction::FireEmployee(
                        game_state_snapshot.employees[self.selected_employee].id,
                    ))
                }
                PlayerInputAction::SelectEmployeeForRaise => {
                    pending_player_action.0 = Some(PlayerAction::GiveRaise(
                        game_state_snapshot.employees[self.selected_employee].id,
                        100,
                    ))
                }
                PlayerInputAction::LaunchPRCampaign => {
                    pending_player_action.0 = Some(PlayerAction::LaunchPRCampaign)
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
                    .employees
                    .iter()
                    .filter(|emp| emp.employment_status == EmploymentStatus::Active)
                    .fold(0, |sum, val| sum + val.salary)
            );
            let stats_widget =
                Paragraph::new(Text::raw(stats)).style(Style::default().fg(Color::White));
            frame.render_widget(stats_widget, inner_chunks[0]);

            let employee_lines = game_state_snapshot
                .employees
                .iter()
                .enumerate()
                .map(|(i, emp)| {
                    let line = format!(
                        "{} - Satisfaction: {:.2}, Employment Status: {:?}",
                        emp.name, emp.satisfaction, emp.employment_status
                    );
                    if i == self.selected_employee {
                        Line::from(Span::styled(
                            line,
                            Style::default().fg(Color::White).bg(Color::Blue),
                        ))
                    } else {
                        Line::from(Span::styled(line, Style::default().fg(Color::Yellow)))
                    }
                })
                .collect::<Vec<Line>>();

            let employees_widget = Paragraph::new(employee_lines)
                .block(Block::default().borders(Borders::ALL).title("Employees"));

            frame.render_widget(employees_widget, inner_chunks[1]);

            let help_line = Line::from(vec![
                Span::styled("[W/↑] Up  ", Style::default().fg(Color::Yellow)),
                Span::styled("[S/↓] Down  ", Style::default().fg(Color::Yellow)),
                Span::styled("[A/←] Left  ", Style::default().fg(Color::Yellow)),
                Span::styled("[D/→] Right  ", Style::default().fg(Color::Yellow)),
                Span::raw(" | "),
                Span::styled("[F] Fire  ", Style::default().fg(Color::Red)),
                Span::styled("[R] Raise  ", Style::default().fg(Color::Green)),
                Span::styled("[L] PR Campaign  ", Style::default().fg(Color::Cyan)),
                Span::styled("[P] Do Nothing  ", Style::default().fg(Color::White)),
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
