use crate::systems::ServerEventSender;
use bevy::prelude::{Changed, Query, Res};
use shared::ServerEvent::UpdatedWorldState;
use shared::{InternalEntity, Player, Position, WorldState};
use tracing::{debug, error};

pub fn sync_world_state(
    channel: Res<ServerEventSender>,
    player_query: Query<
        // component
        (&Player, &InternalEntity, &Position),
        // filter
        Changed<Position>,
    >,
) {
    if player_query.get_single().is_err() {
        return;
    }

    let (_player, internal_entity, position) = player_query.single();
    match channel
        .tx_server_events
        .try_send(UpdatedWorldState(WorldState {
            player_1_internal_entity: internal_entity.clone(),
            player_1_position: position.clone(),
        })) {
        Ok(_) => {
            debug!("Updated world state");
        }
        Err(e) => {
            error!("Failed to update world state: {:?}", e);
        }
    }
}
