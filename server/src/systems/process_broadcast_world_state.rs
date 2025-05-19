use crate::NeedsWorldBroadcast;
use crate::systems::ServerEventSender;
use bevy::prelude::{Res, ResMut};
use shared::{ServerEvent, ServerGameState, ServerHistoryState};

pub fn process_broadcast_world_state(
    mut needs_broadcast: ResMut<NeedsWorldBroadcast>,
    server_history_state: Res<ServerHistoryState>,
    server_game_state: Res<ServerGameState>,
    server_event_sender: Res<ServerEventSender>,
) {
    if needs_broadcast.0 {
        needs_broadcast.0 = false;
    } else {
        return;
    }

    // Only send if there's a connected player and we're due to broadcast
    if let Some(player) = server_game_state.game_state.players.first() {
        if player.id.is_none() {
            return;
        }
    } else {
        panic!("Should be at least one player entity, this is weird")
    }

    let _ = server_event_sender
        .tx_server_events
        .try_send(ServerEvent::FullState(server_game_state.game_state.clone()));

    let _ = server_event_sender
        .tx_server_events
        .try_send(ServerEvent::HistoryState(
            server_history_state.history_state.clone(),
        ));
}
