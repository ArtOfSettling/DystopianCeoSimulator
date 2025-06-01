use crate::Instances;
use bevy::prelude::ResMut;
use shared::ServerEvent;

pub fn process_broadcast_world_state(mut instances: ResMut<Instances>) {
    for (_game_id, instance) in instances.active_instances.iter_mut() {
        if instance.needs_broadcast {
            instance.needs_broadcast = false;
        } else {
            continue;
        }

        let _ = instance.tx_to_clients.try_send(ServerEvent::FullState(
            instance.instance_game.game_state.clone(),
        ));

        let _ = instance.tx_to_clients.try_send(ServerEvent::HistoryState(
            instance.instance_game.history_state.clone(),
        ));
    }
}
