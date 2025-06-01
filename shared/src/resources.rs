use crate::{ClientActionCommand, ClientMessage, OperatorMode};
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
    Rejected(String),
    Reconnecting { attempts: u64, next_attempt_in: u64 },
}

#[derive(Resource)]
pub struct OperatorModeResource {
    pub operator_mode: OperatorMode,
}

#[derive(Resource, Debug)]
pub enum PendingAction {
    FireEmployee(Uuid),
    GiveRaise(Uuid, f64),
    LaunchPRCampaign,
    DoNothing,
}

#[derive(Resource, Default, Debug)]
pub struct PendingPlayerAction(pub Option<ClientActionCommand>);

#[derive(Resource, Default, Debug)]
pub struct PendingClientMessage(pub Option<ClientMessage>);
