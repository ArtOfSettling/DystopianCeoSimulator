use crate::systems::ServerEventsReceiver;
use bevy::ecs::system::SystemState;
use bevy::prelude::{Commands, QueryState, ResMut, World};
use shared::{Player, Position, ServerEvent};
use tracing::{debug, info};

pub fn process_server_events(
    world: &mut World,
    player_query: &mut QueryState<(&Player, &mut Position)>,
    params: &mut SystemState<(ResMut<ServerEventsReceiver>, Commands)>,
) {
    {
        let (server_events_receiver, _) = params.get_mut(world);

        let received = server_events_receiver.rx_server_events.try_recv();
        if received.is_err() {
            return;
        }

        match received.unwrap() {
            ServerEvent::None => info!("Client has an empty server event"),
            ServerEvent::UpdatedWorldState(updated_world_state) => {
                let found_player = player_query
                    .iter(world)
                    .any(|(player, _position)| return player.id == updated_world_state.player_1.id);

                if found_player {
                    debug!(
                        "Client is not spawning a new player because a player with this id already exists, updating position"
                    );
                    player_query
                        .iter_mut(world)
                        .for_each(|(_player, mut position)| {
                            position.x = updated_world_state.player_1_position.x;
                            position.y = updated_world_state.player_1_position.y;
                        });
                    return;
                }

                info!(
                    "Client is spawning a new Player {:?} at Position: {:?}",
                    updated_world_state.player_1, updated_world_state.player_1_position
                );
                world.spawn((
                    updated_world_state.player_1,
                    updated_world_state.player_1_position,
                ));
            }
        }
    }
}
