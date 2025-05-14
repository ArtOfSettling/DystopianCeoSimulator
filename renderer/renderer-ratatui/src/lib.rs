mod org_detailed_view;
mod org_view;

use crate::org_detailed_view::draw_detailed_org_view;
use crate::org_view::draw_org_view;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Res, ResMut};
use input_api::{PendingPlayerInputAction, PlayerInputAction};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::Paragraph};
use renderer_api::{Renderer, RendererResource};
use shared::{
    ChildSnapshot, GameStateSnapshot, OrganizationSnapshot, PendingPlayerAction, PetSnapshot,
    PlayerAction,
};
use std::io;
use tracing::{debug, error};

#[derive(Default)]
pub struct RatatuiRendererPlugin {}

enum UiMode {
    OrgView,
    DetailedOrgView,
}

pub struct RatatuiRenderer {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    selected_index: usize,
    selected_organization: usize,
    ui_mode: UiMode,
}

impl Plugin for RatatuiRendererPlugin {
    fn build(&self, app: &mut App) {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend).unwrap();
        let renderer = RatatuiRenderer {
            terminal,
            selected_index: 0,
            selected_organization: 0,
            ui_mode: UiMode::OrgView,
        };

        app.insert_resource(RendererResource::new(Box::new(renderer)))
            .add_systems(Update, render_system);
    }
}

fn total_selectable_items(
    org: &OrganizationSnapshot,
    child_snapshots: &[ChildSnapshot],
    pet_snapshots: &[PetSnapshot],
) -> usize {
    let mut total = 0;
    for emp in &org.employees {
        total += 1; // the employee
        total += child_snapshots
            .iter()
            .filter(|child| emp.children_ids.contains(&child.id))
            .count();
        total += pet_snapshots
            .iter()
            .filter(|pet| emp.pet_ids.contains(&pet.id))
            .count();
    }
    total
}

impl Renderer for RatatuiRenderer {
    fn render(
        &mut self,
        game_state_snapshot: Res<GameStateSnapshot>,
        mut pending_player_input_action: ResMut<PendingPlayerInputAction>,
        mut pending_player_action: ResMut<PendingPlayerAction>,
    ) {
        match self.ui_mode {
            UiMode::OrgView => match &pending_player_input_action.0 {
                Some(input_action) => match input_action {
                    PlayerInputAction::MenuSelect => {
                        self.selected_organization = self.selected_index;
                        self.ui_mode = UiMode::DetailedOrgView;
                        self.selected_index = 0;
                    }
                    PlayerInputAction::MenuUp => {
                        if self.selected_index == 0 {
                            self.selected_index = game_state_snapshot.organizations.len() - 1;
                        } else {
                            self.selected_index -= 1;
                        }
                    }
                    PlayerInputAction::MenuDown => {
                        self.selected_index += 1;
                        if self.selected_index >= game_state_snapshot.organizations.len() {
                            self.selected_index = 0;
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            UiMode::DetailedOrgView => match &pending_player_input_action.0 {
                Some(input_action) => match input_action {
                    PlayerInputAction::GoBack => {
                        self.selected_organization = 0;
                        self.ui_mode = UiMode::OrgView;
                    }
                    PlayerInputAction::MenuSelect => {}
                    PlayerInputAction::MenuUp => {
                        if self.selected_index == 0 {
                            self.selected_index = total_selectable_items(
                                &game_state_snapshot.organizations[self.selected_organization],
                                &game_state_snapshot.children,
                                &game_state_snapshot.pets,
                            ) - 1;
                        } else {
                            self.selected_index -= 1;
                        }
                    }
                    PlayerInputAction::MenuDown => {
                        self.selected_index += 1;
                        if self.selected_index
                            >= total_selectable_items(
                                &game_state_snapshot.organizations[self.selected_organization],
                                &game_state_snapshot.children,
                                &game_state_snapshot.pets,
                            )
                        {
                            self.selected_index = 0;
                        }
                    }

                    PlayerInputAction::SelectEmployeeToFire => {
                        if let Some(employee) = game_state_snapshot.organizations
                            [self.selected_organization]
                            .employees
                            .get(self.selected_index)
                        {
                            pending_player_action.0 = Some(PlayerAction::FireEmployee(employee.id));
                        }
                    }
                    PlayerInputAction::SelectEmployeeForRaise => {
                        if let Some(employee) = game_state_snapshot.organizations
                            [self.selected_organization]
                            .employees
                            .get(self.selected_index)
                        {
                            pending_player_action.0 =
                                Some(PlayerAction::GiveRaise(employee.id, 100));
                        }
                    }
                    PlayerInputAction::LaunchPRCampaign => {
                        pending_player_action.0 = Some(PlayerAction::LaunchPRCampaign)
                    }
                    PlayerInputAction::DoNothing => {
                        pending_player_action.0 = Some(PlayerAction::DoNothing)
                    }
                    PlayerInputAction::PromoteToVP => {
                        if let Some(employee) = game_state_snapshot.organizations
                            [self.selected_organization]
                            .employees
                            .get(self.selected_index)
                        {
                            if let Some(org) = game_state_snapshot
                                .organizations
                                .iter()
                                .find(|org| org.employees.iter().any(|emp| emp.id == employee.id))
                            {
                                pending_player_action.0 = Some(PlayerAction::PromoteToVp {
                                    target_id: org.id,
                                    employee_id: employee.id,
                                });
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
        }

        pending_player_input_action.0 = None;

        match self.terminal.draw(|frame| {
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(frame.area());

            match self.ui_mode {
                UiMode::OrgView => {
                    draw_org_view(
                        frame,
                        main_chunks[0],
                        &game_state_snapshot,
                        self.selected_index,
                    );
                }
                UiMode::DetailedOrgView => {
                    draw_detailed_org_view(
                        frame,
                        main_chunks[0],
                        &game_state_snapshot.organizations[self.selected_organization],
                        &game_state_snapshot.pets,
                        &game_state_snapshot.children,
                        self.selected_index,
                    );
                }
            }

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
        };
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
