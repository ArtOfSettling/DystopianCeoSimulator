use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InternalCommand {
    PlayerConnected { player_id: Uuid },
    PlayerDisconnected { player_id: Uuid },
}
