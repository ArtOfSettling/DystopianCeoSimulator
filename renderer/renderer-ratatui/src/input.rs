use crate::navigation::{NavigationAction, NavigationStack};
use crate::routes::{OrganizationList, OrganizationTab, OrganizationView, Route};
use bevy::prelude::ResMut;
use input_api::PlayerInputAction;
use renderer_api::ClientGameState;
use shared::{Budget, PendingPlayerAction, PlayerAction};

fn try_switch_tab(current: &Route) -> Option<Route> {
    match current {
        Route::OrganizationView { data } => {
            let next_tab = match data.tab {
                OrganizationTab::Detail => OrganizationTab::Budget,
                OrganizationTab::Budget => OrganizationTab::Hiring,
                OrganizationTab::Hiring => OrganizationTab::Detail,
            };
            Some(Route::OrganizationView {
                data: OrganizationView {
                    organization_id: data.organization_id,
                    selected_index: 0,
                    tab: next_tab,
                    marketing: data.marketing,
                    rnd: data.rnd,
                    training: data.training,
                },
            })
        }
        _ => None,
    }
}

pub fn handle_input(
    player_input_action: PlayerInputAction,
    nav: &mut NavigationStack,
    client_game_state: &ClientGameState,
    pending_player_action: &mut ResMut<PendingPlayerAction>,
) -> bool {
    if let PlayerInputAction::MenuChangeTab = player_input_action {
        if let Some(new_tab) = try_switch_tab(nav.current()) {
            NavigationAction::Switch(new_tab).apply(nav);
        }
    }

    let (company_id, _company) = client_game_state.companies.iter().next().unwrap();
    match player_input_action {
        PlayerInputAction::Quit => return NavigationAction::Quit.apply(nav),
        PlayerInputAction::MenuBack => return NavigationAction::Pop.apply(nav),
        PlayerInputAction::MenuSelect => {
            if let Route::OrganizationList { data } = nav.current() {
                NavigationAction::Push(Route::OrganizationView {
                    data: OrganizationView {
                        selected_index: 0,
                        organization_id: client_game_state
                            .ordered_organizations_of_company
                            .get(company_id)
                            .map(|organizations| organizations[data.selected_index].clone())
                            .unwrap(),
                        tab: OrganizationTab::Detail,
                        marketing: client_game_state
                            .organizations
                            .get(
                                &client_game_state
                                    .ordered_organizations_of_company
                                    .get(company_id)
                                    .map(|organizations| organizations[data.selected_index].clone())
                                    .unwrap(),
                            )
                            .unwrap()
                            .budget
                            .marketing,
                        rnd: client_game_state
                            .organizations
                            .get(
                                &client_game_state
                                    .ordered_organizations_of_company
                                    .get(company_id)
                                    .map(|organizations| organizations[data.selected_index].clone())
                                    .unwrap(),
                            )
                            .unwrap()
                            .budget
                            .rnd,
                        training: client_game_state
                            .organizations
                            .get(
                                &client_game_state
                                    .ordered_organizations_of_company
                                    .get(company_id)
                                    .map(|organizations| organizations[data.selected_index].clone())
                                    .unwrap(),
                            )
                            .unwrap()
                            .budget
                            .training,
                    },
                })
                .apply(nav);
            }
        }
        _ => {}
    }

    pending_player_action.0 = match nav.current_mut() {
        Route::OrganizationList { data } => {
            data.handle_input(player_input_action, client_game_state)
        }
        Route::OrganizationView { data } => {
            data.handle_input(player_input_action, client_game_state)
        }
    };

    true
}

pub trait InputHandler {
    fn handle_input(
        &mut self,
        action: PlayerInputAction,
        client_game_state: &ClientGameState,
    ) -> Option<PlayerAction>;
}

impl InputHandler for OrganizationList {
    fn handle_input(
        &mut self,
        action: PlayerInputAction,
        client_game_state: &ClientGameState,
    ) -> Option<PlayerAction> {
        let (company_id, _company) = client_game_state.companies.iter().next().unwrap();
        match action {
            PlayerInputAction::DoNothing => Some(PlayerAction::DoNothing),
            PlayerInputAction::MenuUp => {
                self.selected_index = self.selected_index.saturating_sub(1);
                None
            }
            PlayerInputAction::MenuDown => {
                self.selected_index = (self.selected_index + 1).min(
                    client_game_state
                        .ordered_organizations_of_company
                        .get(company_id)
                        .map(|organizations| organizations.len() - 1)
                        .unwrap_or(0),
                );
                None
            }
            PlayerInputAction::LaunchPRCampaign => Some(PlayerAction::LaunchPRCampaign),
            _ => None,
        }
    }
}

impl InputHandler for OrganizationView {
    fn handle_input(
        &mut self,
        action: PlayerInputAction,
        client_game_state: &ClientGameState,
    ) -> Option<PlayerAction> {
        match self.tab {
            OrganizationTab::Detail => match action {
                PlayerInputAction::MenuDown => {
                    let employee_count = client_game_state
                        .ordered_employees_of_organization
                        .get(&self.organization_id)
                        .map(|organization| organization.len())
                        .unwrap_or(0);
                    self.selected_index = (self.selected_index + 1).min(employee_count - 1);
                    None
                }
                PlayerInputAction::MenuUp => {
                    self.selected_index = self.selected_index.saturating_sub(1);
                    None
                }
                PlayerInputAction::SelectEmployeeForPromotionToVP => {
                    if let Some(employee_id) = client_game_state
                        .ordered_employees_of_organization
                        .get(&self.organization_id)?
                        .get(self.selected_index)
                    {
                        Some(PlayerAction::PromoteToVp {
                            organization_id: self.organization_id,
                            employee_id: *employee_id,
                        })
                    } else {
                        None
                    }
                }

                PlayerInputAction::SelectEmployeeForRaise => {
                    if let Some(employee_id) = client_game_state
                        .ordered_employees_of_organization
                        .get(&self.organization_id)?
                        .get(self.selected_index)
                    {
                        Some(PlayerAction::GiveRaise {
                            employee_id: *employee_id,
                            amount: 1_000,
                        })
                    } else {
                        None
                    }
                }

                PlayerInputAction::SelectEmployeeToFire => {
                    if let Some(employee_id) = client_game_state
                        .ordered_employees_of_organization
                        .get(&self.organization_id)?
                        .get(self.selected_index)
                    {
                        Some(PlayerAction::FireEmployee {
                            employee_id: *employee_id,
                        })
                    } else {
                        None
                    }
                }

                _ => None,
            },
            OrganizationTab::Budget => match action {
                PlayerInputAction::MenuDown => {
                    let budget_entries = 3;
                    self.selected_index = (self.selected_index + 1).min(budget_entries - 1);
                    None
                }
                PlayerInputAction::MenuUp => {
                    self.selected_index = self.selected_index.saturating_sub(1);
                    None
                }
                PlayerInputAction::MenuDecrement => {
                    match self.selected_index {
                        0 => self.marketing -= 1,
                        1 => self.rnd -= 1,
                        2 => self.training -= 1,
                        _ => {}
                    }
                    None
                }
                PlayerInputAction::MenuIncrement => {
                    match self.selected_index {
                        0 => self.marketing += 1,
                        1 => self.rnd += 1,
                        2 => self.training += 1,
                        _ => {}
                    }
                    None
                }
                PlayerInputAction::MenuCommit => Some(PlayerAction::UpdateBudget {
                    organization_id: self.organization_id,
                    organization_budget: Budget {
                        marketing: self.marketing,
                        rnd: self.rnd,
                        training: self.training,
                    },
                }),
                _ => None,
            },
            OrganizationTab::Hiring => match action {
                PlayerInputAction::MenuDown => {
                    let unemployed_count = client_game_state.ordered_unemployed_entities.len();
                    self.selected_index = (self.selected_index + 1).min(unemployed_count - 1);
                    None
                }

                PlayerInputAction::MenuUp => {
                    self.selected_index = self.selected_index.saturating_sub(1);
                    None
                }

                PlayerInputAction::SelectEmployeeToHire => {
                    if let Some(employee_id) = client_game_state
                        .ordered_unemployed_entities
                        .get(self.selected_index)
                    {
                        Some(PlayerAction::HireEmployee {
                            employee_id: *employee_id,
                            organization_id: self.organization_id,
                        })
                    } else {
                        None
                    }
                }

                _ => None,
            },
        }
    }
}
