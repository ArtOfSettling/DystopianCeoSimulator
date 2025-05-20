use std::collections::HashMap;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum EntityKind {
    Player,
    Company,
    Organization,
}

#[derive(Debug, Clone)]
pub struct SelectedEntityKindData {
    pub selected_index: usize,
    pub entity_count: usize,
}

#[derive(Debug, Clone)]
pub struct DashboardData {
    pub entity_kind: EntityKind,
    pub selected_entity_kind_data: HashMap<EntityKind, SelectedEntityKindData>,
}

impl DashboardData {
    pub fn selected_index(&self) -> Option<usize> {
        self.selected_entity_kind_data
            .get(&self.entity_kind)
            .map(|v| v.selected_index)
    }

    pub fn move_selection(&mut self, delta: isize, list_len: usize) {
        if list_len == 0 {
            return;
        }

        let data = self.selected_entity_kind_data.get_mut(&self.entity_kind);

        if let Some(selection) = data {
            let current = selection.selected_index as isize;
            let mut new_index = current - delta;

            // Wrap around properly
            if new_index < 0 {
                new_index = 0;
            } else if new_index >= list_len as isize {
                new_index = list_len as isize - 1;
            }

            selection.selected_index = new_index as usize;
        }
    }

    pub fn cycle_kind(&mut self) {
        self.entity_kind = match self.entity_kind {
            EntityKind::Player => EntityKind::Company,
            EntityKind::Company => EntityKind::Organization,
            EntityKind::Organization => EntityKind::Player,
        };
    }
}

#[derive(Debug, Clone)]
pub enum Route {
    Dashboard { data: DashboardData },
}
