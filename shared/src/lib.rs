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
pub struct Player {
    pub id: Uuid,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Player {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldState {
    pub player_1: Player,
    pub player_1_position: Position,
}
