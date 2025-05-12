use serde::{Deserialize, Serialize};
use shared::components::InternalEntity;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InternalCommand {
    PlayerConnected(InternalEntity),
    PlayerDisconnected(InternalEntity),
}
