use crate::{GameState, HistoryState, OperatorMode};
use bevy::prelude::Resource;
use uuid::Uuid;

#[derive(Resource)]
pub struct ConnectionStateResource {
    pub connection_state: ConnectionState,
}

#[derive(Debug, Clone)]
pub enum ConnectionState {
    Connecting,
    Connected,
    Disconnected,
    Error(String),
    Reconnecting { attempts: u64, next_attempt_in: u64 },
}

#[derive(Resource)]
pub struct OperatorModeResource {
    pub operator_mode: OperatorMode,
}

#[derive(Resource)]
pub struct ServerGameState {
    pub game_state: GameState,
}

#[derive(Resource)]
pub struct ServerHistoryState {
    pub history_state: HistoryState,
}

impl Default for ServerGameState {
    fn default() -> Self {
        Self {
            game_state: GameState {
                week: 0,
                players: Default::default(),
                companies: Default::default(),
                organizations: Default::default(),
                entities: Default::default(),
            },
        }
    }
}

impl Default for ServerHistoryState {
    fn default() -> Self {
        Self {
            history_state: HistoryState {
                players: Default::default(),
                organizations: Default::default(),
                companies: Default::default(),
            },
        }
    }
}

#[derive(Resource, Debug)]
pub enum PendingAction {
    FireEmployee(Uuid),
    GiveRaise(Uuid, f64),
    LaunchPRCampaign,
    DoNothing,
}
