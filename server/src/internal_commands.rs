use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InternalCommand {
    OperatorConnected { id: Uuid },
    OperatorDisconnected { id: Uuid },

    DashboardViewerConnected { id: Uuid },
    DashboardViewerDisconnected { id: Uuid },
}
