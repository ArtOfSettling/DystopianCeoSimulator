use crate::GameClientActionCommand;
use async_channel::{Receiver, Sender};
use shared::{OperatorMode, ServerEvent};
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum InternalCommand {
    Connected {
        id: Uuid,
        addr: SocketAddr,
        game_id: Uuid,
        operator_mode: OperatorMode,
        tx_to_clients: Sender<ServerEvent>,
        rx_from_clients: Receiver<GameClientActionCommand>,
    },
    Disconnected {
        id: Uuid,
        game_id: Uuid,
    },
}
