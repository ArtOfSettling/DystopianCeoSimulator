use crate::navigation::{NavigationAction, NavigationStack};
use crate::routes::{OrganizationList, OrganizationTab, OrganizationView, Route};
use bevy::prelude::ResMut;
use input_api::PlayerInputAction;
use shared::{GameStateSnapshot, PendingPlayerAction, PlayerAction};

fn try_switch_tab(current: &Route) -> Option<Route> {
    match current {
        Route::OrganizationView { data } => {
            let next_tab = match data.tab {
                OrganizationTab::Detail => OrganizationTab::Hiring,
                OrganizationTab::Hiring => OrganizationTab::Detail,
            };
            Some(Route::OrganizationView {
                data: OrganizationView {
                    organization_id: data.organization_id,
                    selected_index: 0,
                    tab: next_tab,
                },
            })
        }
        _ => None,
    }
}

pub fn handle_input(
    player_input_action: PlayerInputAction,
    nav: &mut NavigationStack,
    game_state_snapshot: &GameStateSnapshot,
    pending_player_action: &mut ResMut<PendingPlayerAction>,
) -> bool {
    if let PlayerInputAction::MenuChangeTab = player_input_action {
        if let Some(new_tab) = try_switch_tab(nav.current()) {
            NavigationAction::Switch(new_tab).apply(nav);
        }
    }

    match player_input_action {
        PlayerInputAction::Quit => return NavigationAction::Quit.apply(nav),
        PlayerInputAction::MenuBack => return NavigationAction::Pop.apply(nav),
        PlayerInputAction::MenuSelect => if let Route::OrganizationList { data } = nav.current() {
            NavigationAction::Push(Route::OrganizationView {
                data: OrganizationView {
                    selected_index: 0,
                    organization_id: game_state_snapshot
                        .organizations
                        .get(data.selected_index)
                        .unwrap()
                        .id,
                    tab: OrganizationTab::Detail,
                },
            })
            .apply(nav);
        },
        _ => {}
    }

    pending_player_action.0 = match nav.current_mut() {
        Route::OrganizationList { data } => {
            data.handle_input(player_input_action, game_state_snapshot)
        }
        Route::OrganizationView { data } => {
            data.handle_input(player_input_action, game_state_snapshot)
        }
    };

    true
}

pub trait InputHandler {
    fn handle_input(
        &mut self,
        action: PlayerInputAction,
        game_state_snapshot: &GameStateSnapshot,
    ) -> Option<PlayerAction>;
}

impl InputHandler for OrganizationList {
    fn handle_input(
        &mut self,
        action: PlayerInputAction,
        game_state_snapshot: &GameStateSnapshot,
    ) -> Option<PlayerAction> {
        match action {
            PlayerInputAction::DoNothing => Some(PlayerAction::DoNothing),
            PlayerInputAction::MenuUp => {
                self.selected_index = self.selected_index.saturating_sub(1);
                None
            }
            PlayerInputAction::MenuDown => {
                self.selected_index =
                    (self.selected_index + 1).min(game_state_snapshot.organizations.len() - 1);
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
        game_state_snapshot: &GameStateSnapshot,
    ) -> Option<PlayerAction> {
        match action {
            PlayerInputAction::DoNothing => Some(PlayerAction::DoNothing),
            PlayerInputAction::MenuDown => {
                let employee_count = game_state_snapshot
                    .organizations
                    .iter()
                    .find(|organization| organization.id == self.organization_id)
                    .map(|organization| organization.employees.len())
                    .unwrap_or(0);
                self.selected_index = (self.selected_index + 1).min(employee_count - 1);
                None
            }
            PlayerInputAction::MenuUp => {
                self.selected_index = self.selected_index.saturating_sub(1);
                None
            }
            PlayerInputAction::LaunchPRCampaign => Some(PlayerAction::LaunchPRCampaign),
            PlayerInputAction::SelectEmployeeForPromotionToVP => {
                let org_snapshot = game_state_snapshot
                    .organizations
                    .iter()
                    .find(|org| org.id == self.organization_id)
                    .unwrap();
                let employee = &org_snapshot.employees[self.selected_index];
                Some(PlayerAction::PromoteToVp {
                    organization_id: self.organization_id,
                    employee_id: employee.id,
                })
            }

            PlayerInputAction::SelectEmployeeForRaise => {
                let org_snapshot = game_state_snapshot
                    .organizations
                    .iter()
                    .find(|org| org.id == self.organization_id)
                    .unwrap();
                let employee = &org_snapshot.employees[self.selected_index];
                Some(PlayerAction::GiveRaise {
                    employee_id: employee.id,
                    amount: 1_000,
                })
            }

            PlayerInputAction::SelectEmployeeToFire => {
                let org_snapshot = game_state_snapshot
                    .organizations
                    .iter()
                    .find(|org| org.id == self.organization_id)
                    .unwrap();
                let employee = &org_snapshot.employees[self.selected_index];
                Some(PlayerAction::FireEmployee {
                    employee_id: employee.id,
                })
            }

            _ => None,
        }
    }
}
