use crate::Instances;
use bevy::prelude::ResMut;

pub fn process_clear_needs_state_update(mut game_instances: ResMut<Instances>) {
    for (_game_id, instance) in game_instances.active_instances.iter_mut() {
        instance.needs_state_update = false;
    }
}
