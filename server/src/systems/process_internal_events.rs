use crate::NeedsWorldBroadcast;
use crate::systems::{FanOutEventReceiver, apply_event};
use bevy::prelude::{Res, ResMut};
use shared::{ServerGameState, ServerHistoryState};
use tracing::info;

pub fn process_internal_events(
    fan_out_event_receiver: Res<FanOutEventReceiver>,
    mut server_game_state: ResMut<ServerGameState>,
    mut server_history_state: ResMut<ServerHistoryState>,
    mut needs_world_broadcast: ResMut<NeedsWorldBroadcast>,
) {
    while let Ok(event) = fan_out_event_receiver.rx_fan_out_events.try_recv() {
        info!("Processing event: {:?}", event);
        apply_event(
            &event,
            &mut server_game_state,
            &mut server_history_state,
            &mut needs_world_broadcast,
        );
    }
}
