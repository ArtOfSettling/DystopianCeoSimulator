use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Res, ResMut};
use input_api::{PendingPlayerInputAction, PlayerInputAction};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Wrap};
use ratatui::{Frame, Terminal, backend::CrosstermBackend, widgets::Paragraph};
use renderer_api::{Renderer, RendererResource};
use shared::PlayerAction::{
    DoNothing, FireEmployee, GiveRaise, HireEmployee, LaunchPRCampaign, PromoteToVp,
};
use shared::{
    AnimalSnapshot, EmployeeSnapshot, GameStateSnapshot, HumanSnapshot, OrganizationSnapshot,
    PendingPlayerAction, UnemployedSnapshot,
};
use std::io;
use tracing::{debug, error};
use uuid::Uuid;

#[derive(Default)]
pub struct RatatuiRendererPlugin {}

#[derive(Debug)]
pub enum AppPage {
    OrgList {
        selected_index: usize,
    },
    OrgDetail {
        org_id: Uuid,
        selected_index: usize,
    },
    Hiring {
        selected_index: usize,
        org_id: Option<Uuid>,
    },
}

pub struct RatatuiRenderer {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    pub current_page: AppPage,
}

impl Plugin for RatatuiRendererPlugin {
    fn build(&self, app: &mut App) {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend).unwrap();
        let renderer = RatatuiRenderer {
            terminal,
            current_page: AppPage::OrgList { selected_index: 0 },
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
        let player_input_action = pending_player_input_action.0.clone();
        pending_player_input_action.0 = None;

        match player_input_action {
            None => {}
            Some(player_input_action) => match self.current_page {
                AppPage::OrgList {
                    ref mut selected_index,
                } => match player_input_action {
                    PlayerInputAction::DoNothing => pending_player_action.0 = Some(DoNothing),
                    PlayerInputAction::MenuUp => *selected_index = selected_index.saturating_sub(1),
                    PlayerInputAction::MenuDown => {
                        *selected_index =
                            (*selected_index + 1).min(game_state_snapshot.organizations.len() - 1)
                    }
                    PlayerInputAction::MenuSelect => {
                        self.current_page = AppPage::OrgDetail {
                            org_id: game_state_snapshot.organizations[*selected_index].id,
                            selected_index: 0,
                        }
                    }
                    PlayerInputAction::SelectEmployeeToHire => {
                        self.current_page = AppPage::Hiring {
                            org_id: Some(game_state_snapshot.organizations[*selected_index].id),
                            selected_index: 0,
                        }
                    }
                    PlayerInputAction::LaunchPRCampaign => {
                        pending_player_action.0 = Some(LaunchPRCampaign)
                    }
                    _ => {}
                },

                AppPage::OrgDetail {
                    org_id,
                    ref mut selected_index,
                } => match player_input_action {
                    PlayerInputAction::DoNothing => pending_player_action.0 = Some(DoNothing),
                    PlayerInputAction::GoBack => {
                        self.current_page = AppPage::OrgList {
                            selected_index: game_state_snapshot
                                .organizations
                                .iter()
                                .position(|item| item.id == org_id)
                                .unwrap_or(0),
                        }
                    }
                    PlayerInputAction::MenuUp => *selected_index = selected_index.saturating_sub(1),
                    PlayerInputAction::MenuDown => *selected_index += 1,
                    PlayerInputAction::SelectEmployeeForPromotionToVP => {
                        let org_snapshot = game_state_snapshot
                            .organizations
                            .iter()
                            .find(|org| org.id == org_id)
                            .unwrap();
                        let employee = &org_snapshot.employees[*selected_index];
                        pending_player_action.0 = Some(PromoteToVp {
                            organization_id: org_id,
                            employee_id: employee.id,
                        })
                    }
                    PlayerInputAction::SelectEmployeeForRaise => {
                        let org_snapshot = game_state_snapshot
                            .organizations
                            .iter()
                            .find(|org| org.id == org_id)
                            .unwrap();
                        let employee = &org_snapshot.employees[*selected_index];
                        pending_player_action.0 = Some(GiveRaise {
                            employee_id: employee.id,
                            amount: 1_000,
                        })
                    }
                    PlayerInputAction::SelectEmployeeToFire => {
                        let org_snapshot = game_state_snapshot
                            .organizations
                            .iter()
                            .find(|org| org.id == org_id)
                            .unwrap();
                        let employee = &org_snapshot.employees[*selected_index];
                        pending_player_action.0 = Some(FireEmployee {
                            employee_id: employee.id,
                        })
                    }
                    _ => {}
                },

                AppPage::Hiring {
                    ref mut selected_index,
                    org_id,
                } => match player_input_action {
                    PlayerInputAction::DoNothing => pending_player_action.0 = Some(DoNothing),
                    PlayerInputAction::GoBack => {
                        self.current_page = AppPage::OrgList {
                            selected_index: game_state_snapshot
                                .organizations
                                .iter()
                                .position(|item| Some(item.id) == org_id)
                                .unwrap_or(0),
                        }
                    }
                    PlayerInputAction::MenuUp => *selected_index = selected_index.saturating_sub(1),
                    PlayerInputAction::MenuDown => *selected_index += 1,
                    PlayerInputAction::SelectEmployeeToHire => {
                        let employee_id = match &game_state_snapshot.unemployed[*selected_index] {
                            UnemployedSnapshot::UnemployedAnimalSnapshot(animal) => animal.id,
                            UnemployedSnapshot::UnemployedHumanSnapshot(human) => human.id,
                        };
                        pending_player_action.0 = Some(HireEmployee {
                            organization_id: org_id.unwrap(),
                            employee_id,
                        })
                    }
                    _ => {}
                },
            },
        }

        match self.terminal.draw(|frame| {
            let outer_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(frame.area());

            let main_area = outer_chunks[0];
            let footer_area = outer_chunks[1];

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(main_area);

            let left_pane = chunks[0];
            let right_pane = chunks[1];

            draw_financial_summary(frame, footer_area, &game_state_snapshot);

            match &self.current_page {
                AppPage::OrgList { selected_index } => {
                    let org = game_state_snapshot.organizations.get(*selected_index);
                    draw_org_list(
                        frame,
                        left_pane,
                        &game_state_snapshot.organizations,
                        *selected_index,
                    );
                    if let Some(org) = org {
                        draw_org_summary(frame, right_pane, org);
                    }
                }
                AppPage::OrgDetail {
                    org_id,
                    selected_index,
                } => {
                    let org = game_state_snapshot
                        .organizations
                        .iter()
                        .find(|o| o.id == *org_id);
                    if let Some(org) = org {
                        draw_employee_list(frame, left_pane, org, *selected_index);
                        if let Some(emp) = org.employees.get(*selected_index) {
                            draw_employee_details(
                                frame,
                                right_pane,
                                game_state_snapshot.week as i32,
                                emp,
                                &game_state_snapshot.pets,
                                &game_state_snapshot.humans,
                            );
                        }
                    }
                }
                AppPage::Hiring {
                    selected_index,
                    org_id: current_org,
                } => {
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
                            game_state_snapshot
                                .organizations
                                .iter()
                                .find(|o| Some(o.id) == current_org.clone())
                                .unwrap(),
                        );
                    }
                }
            }
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

fn draw_financial_summary(frame: &mut Frame, rect: Rect, game_state_snapshot: &GameStateSnapshot) {
    let lines = vec![
        format!("Week: {}", game_state_snapshot.week),
        format!("Cash: ${}", game_state_snapshot.financials.actual_cash),
        format!(
            "Income: ${}",
            game_state_snapshot.financials.this_weeks_income
        ),
        format!(
            "Expenses: ${}",
            game_state_snapshot.financials.this_weeks_expenses
        ),
        format!(
            "Net Profit: ${}",
            game_state_snapshot.financials.this_weeks_net_profit
        ),
    ];

    let block = Block::default().title("Financials").borders(Borders::ALL);

    let paragraph = Paragraph::new(lines.join(" | "))
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, rect);
}

fn draw_org_list(
    frame: &mut Frame,
    rect: Rect,
    org_snapshots: &Vec<OrganizationSnapshot>,
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

    frame.render_stateful_widget(list, rect, &mut state);
}

fn draw_org_summary(frame: &mut Frame, rect: Rect, org_snapshot: &OrganizationSnapshot) {
    let lines = vec![
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
        format!("Employees: {}", org_snapshot.employees.len()),
        format!("Cash: ${}", org_snapshot.financials.actual_cash),
        format!("Income: ${}", org_snapshot.financials.this_weeks_income),
        format!("Expenses: ${}", org_snapshot.financials.this_weeks_expenses),
        format!(
            "Net Profit: ${}",
            org_snapshot.financials.this_weeks_net_profit
        ),
    ];

    let block = Block::default()
        .title("Organization Summary")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(lines.join("\n"))
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, rect);
}

fn draw_employee_list(
    frame: &mut Frame,
    rect: Rect,
    org_snapshot: &OrganizationSnapshot,
    selected_index: usize,
) {
    let items: Vec<ListItem> = org_snapshot
        .employees
        .iter()
        .map(|e| ListItem::new(format!("{} (L{})", e.name, e.level)))
        .collect();

    let mut state = ListState::default();
    state.select(Some(selected_index));

    let list = List::new(items)
        .block(Block::default().title("Employees").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("➤ ");

    frame.render_stateful_widget(list, rect, &mut state);
}

fn draw_employee_details(
    frame: &mut Frame,
    rect: Rect,
    current_week: i32,
    employee_snapshot: &EmployeeSnapshot,
    pets_snapshot: &Vec<AnimalSnapshot>,
    children_snapshot: &Vec<HumanSnapshot>,
) {
    let mut lines = vec![
        format!("Name: {}", employee_snapshot.name),
        format!(
            "Age: {}",
            get_age_description(current_week.saturating_sub(employee_snapshot.week_of_birth) as u32)
        ),
        format!("Type: {:?}", employee_snapshot.entity_type),
        format!("Role: {:?}", employee_snapshot.role),
        format!("Level: {}", employee_snapshot.level),
        format!("Satisfaction: {}", employee_snapshot.satisfaction),
        format!("Salary: ${}", employee_snapshot.salary),
    ];

    // Children
    for child in children_snapshot
        .iter()
        .filter(|c| employee_snapshot.children_ids.contains(&c.id))
    {
        lines.push(format!("  └─ Child: {}", child.name));
    }

    // Pets
    for pet in pets_snapshot
        .iter()
        .filter(|p| employee_snapshot.pet_ids.contains(&p.id))
    {
        lines.push(format!("  └─ Pet: {} ({:?})", pet.name, pet.entity_type));
    }

    let block = Block::default()
        .title("Employee Details")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(lines.join("\n"))
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, rect);
}

fn draw_unemployed_list(
    frame: &mut Frame,
    rect: Rect,
    unemployed_snapshot: &Vec<UnemployedSnapshot>,
    selected_index: usize,
) {
    let items: Vec<ListItem> = unemployed_snapshot
        .iter()
        .map(|p| {
            ListItem::new(format!(
                "{}",
                match p {
                    UnemployedSnapshot::UnemployedAnimalSnapshot(animal) => animal.name.clone(),
                    UnemployedSnapshot::UnemployedHumanSnapshot(human) => human.name.clone(),
                }
            ))
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

    frame.render_stateful_widget(list, rect, &mut state);
}

fn draw_candidate_details(
    frame: &mut Frame,
    rect: Rect,
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

    frame.render_widget(paragraph, rect);
}

fn get_age_description(weeks: u32) -> String {
    let years = weeks / 52;
    let remaining_weeks = weeks % 52;
    format!("{} years, {} weeks", years, remaining_weeks)
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
