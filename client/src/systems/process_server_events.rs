use crate::systems::ServerEventsReceiver;
use bevy::ecs::system::SystemState;
use bevy::prelude::{Commands, ResMut, World};
use shared::{GameStateSnapshot, ServerEvent};
use tracing::info;

pub fn process_server_events(
    world: &mut World,
    params: &mut SystemState<(
        ResMut<ServerEventsReceiver>,
        ResMut<GameStateSnapshot>,
        Commands,
    )>,
) {
    let (server_events_receiver, mut game_state_snapshot, _) = params.get_mut(world);
    let received = server_events_receiver.rx_server_events.try_recv();
    if received.is_err() {
        return;
    }

    match received.unwrap() {
        ServerEvent::None => info!("Client has an empty server event"),
        ServerEvent::FullState(rx_game_state_snapshot) => {
            game_state_snapshot.money = rx_game_state_snapshot.money;
            game_state_snapshot.reputation = rx_game_state_snapshot.reputation;
            game_state_snapshot.employees = rx_game_state_snapshot.employees;
        }
    }
}
