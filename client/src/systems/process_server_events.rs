use crate::systems::ServerEventsReceiver;
use bevy::ecs::system::SystemState;
use bevy::prelude::{Commands, ResMut, World};
use shared::{GameStateSnapshot, ServerEvent};
use tracing::{debug, info};

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
        debug!(
            "server_events_receiver.rx_server_events.try_recv(), {:?}",
            received
        );
        return;
    }

    match received.unwrap() {
        ServerEvent::None => info!("Client has an empty server event"),
        ServerEvent::FullState(rx_game_state_snapshot) => {
            game_state_snapshot.week = rx_game_state_snapshot.week;
            game_state_snapshot.public_opinion = rx_game_state_snapshot.public_opinion;
            game_state_snapshot.reputation = rx_game_state_snapshot.reputation;
            game_state_snapshot.company_public_opinion = rx_game_state_snapshot.company_public_opinion;
            game_state_snapshot.company_reputation = rx_game_state_snapshot.company_reputation;
            game_state_snapshot.financials = rx_game_state_snapshot.financials;
            game_state_snapshot.organizations = rx_game_state_snapshot.organizations;
            game_state_snapshot.pets = rx_game_state_snapshot.pets;
            game_state_snapshot.humans = rx_game_state_snapshot.humans;
            game_state_snapshot.unemployed = rx_game_state_snapshot.unemployed;
        }
    }
}
