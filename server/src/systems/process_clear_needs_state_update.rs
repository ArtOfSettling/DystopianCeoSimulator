use crate::NeedsStateUpdate;
use bevy::prelude::ResMut;

pub fn process_clear_needs_state_update(mut needs_state_update: ResMut<NeedsStateUpdate>) {
    needs_state_update.0 = false;
}
