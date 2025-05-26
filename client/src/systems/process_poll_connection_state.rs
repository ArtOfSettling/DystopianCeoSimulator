use crate::systems::ConnectionStateReceiver;
use bevy::prelude::{Res, ResMut};
use shared::ConnectionStateResource;
use tracing::info;

pub fn process_poll_connection_state(
    mut conn_status: ResMut<ConnectionStateResource>,
    rx_connection_state: Res<ConnectionStateReceiver>,
) {
    while let Ok(state) = rx_connection_state.rx_connection_state.try_recv() {
        conn_status.connection_state = state.clone();
        info!("Connection status changed: {:?}", state);
    }
}
