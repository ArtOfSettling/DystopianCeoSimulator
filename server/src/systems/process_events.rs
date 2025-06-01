use crate::Instances;
use crate::systems::{EventLog, apply_event, write_event_to_log_stream};
use bevy::prelude::ResMut;
use tracing::debug;

pub fn process_events(mut event_log: ResMut<EventLog>, mut instances: ResMut<Instances>) {
    for (game_id, instance) in instances.active_instances.iter_mut() {
        while let Ok(event) = instance.rx_internal_events.try_recv() {
            debug!("Writing event: {:?}", event);
            write_event_to_log_stream(&mut event_log, game_id, event.clone());

            debug!("Processing event: {:?}", event);
            apply_event(&event.internal_event, instance);
        }
    }
}
