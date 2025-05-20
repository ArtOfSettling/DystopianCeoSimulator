use crate::dashboard_viewer::routes::{DashboardData, EntityKind, Route};

#[allow(dead_code)]
pub enum NavigationAction {
    Push(Route),
    Pop,
    Switch(Route),
    Quit,
}

impl NavigationAction {
    pub fn apply(self, nav: &mut NavigationStack) -> bool {
        match self {
            NavigationAction::Push(route) => {
                nav.push(route);
                true
            }
            NavigationAction::Pop => {
                nav.pop();
                true
            }
            NavigationAction::Switch(route) => {
                nav.pop();
                nav.push(route);
                true
            }
            NavigationAction::Quit => false,
        }
    }
}

#[derive(Default)]
pub struct NavigationStack {
    stack: Vec<Route>,
}

impl NavigationStack {
    pub fn new() -> Self {
        Self {
            stack: vec![Route::Dashboard {
                data: DashboardData {
                    entity_kind: EntityKind::Player,
                    selected_entity_kind_data: Default::default(),
                },
            }],
        }
    }

    #[allow(dead_code)]
    pub fn current(&self) -> &Route {
        self.stack
            .last()
            .expect("Navigation stack should never be empty")
    }

    pub fn current_mut(&mut self) -> &mut Route {
        self.stack
            .last_mut()
            .expect("Navigation stack should never be empty")
    }

    pub fn push(&mut self, route: Route) {
        self.stack.push(route);
    }

    pub fn pop(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }
}
