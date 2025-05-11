use crate::systems::ServerEventsReceiver;
use bevy::ecs::system::SystemState;
use bevy::prelude::{Commands, QueryState, ResMut, World};
use shared::{InternalEntity, Player, Position, ServerEvent};
use tracing::info;

pub fn process_server_events(
    world: &mut World,
    player_query: &mut QueryState<(&Player, &InternalEntity, &mut Position)>,
    params: &mut SystemState<(ResMut<ServerEventsReceiver>, Commands)>,
) {
    let (server_events_receiver, _) = params.get_mut(world);

    let received = server_events_receiver.rx_server_events.try_recv();
    if received.is_err() {
        return;
    }

    match received.unwrap() {
        ServerEvent::None => info!("Client has an empty server event"),
        ServerEvent::InitialWorldState(world_state) => {
            info!(
                "Client is spawning a new Player {:?} at Position: {:?}",
                world_state.player_1_internal_entity, world_state.player_1_position
            );
            world.spawn((
                Player {},
                world_state.player_1_internal_entity,
                world_state.player_1_position,
            ));
        }
        ServerEvent::UpdatedWorldState(world_state) => {
            player_query
                .iter(world)
                .any(|(_player, internal_entity, _position)| {
                    return *internal_entity == world_state.player_1_internal_entity;
                });

            info!(
                "Client receiving Player {:?} Update at Position: {:?}",
                world_state.player_1_internal_entity, world_state.player_1_position
            );

            player_query
                .iter_mut(world)
                .for_each(|(_player, _internal_entity, mut position)| {
                    position.x = world_state.player_1_position.x;
                    position.y = world_state.player_1_position.y;
                });
        }
    }
}
