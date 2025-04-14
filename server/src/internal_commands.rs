use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InternalCommand {
    PlayerConnected(Uuid),
    PlayerDisconnected(Uuid),
}
