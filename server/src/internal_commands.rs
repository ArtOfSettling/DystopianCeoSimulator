use crate::GameClientActionCommand;
use crate::systems::ClientInfo;
use async_channel::{Receiver, Sender};
use shared::{OperatorMode, ServerEvent};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum InternalCommand {
    Connected {
        client_info: ClientInfo,
        operator_mode: OperatorMode,
        tx_to_clients: Sender<ServerEvent>,
        rx_from_clients: Receiver<GameClientActionCommand>,
    },
    Disconnected {
        id: Uuid,
        game_id: Uuid,
    },

    CreateGame {
        client_id: Uuid,
        game_name: String,
    },
    ListGames {
        client_id: Uuid,
    },
}
