use crate::systems::ServerEventSender;
use bevy::prelude::Res;

pub fn send_server_commands(channel: Res<ServerEventSender>) {
    let _tx = &channel.tx_server_events;
}
