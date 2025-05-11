use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientCommand {
    PlayerAction(PlayerAction),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerEvent {
    None,

    InitialWorldState(WorldState),
    UpdatedWorldState(WorldState),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlayerAction {
    // Local Movement
    MovePlayerLocalUp,
    MovePlayerLocalRight,
    MovePlayerLocalDown,
    MovePlayerLocalLeft,

    // Room Movement
    MovePlayerRoomUp,
    MovePlayerRoomRight,
    MovePlayerRoomDown,
    MovePlayerRoomLeft,

    // Stairs Movement
    GoDownStairs,
    GoUpStairs,

    ChatMessage(String),
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Player;

#[derive(Component, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalEntity(Uuid);

impl InternalEntity {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldState {
    pub player_1_internal_entity: InternalEntity,
    pub player_1_position: Position,
}
