use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameMetadata {
    pub id: Uuid,
    pub name: String,
    pub created_at: SystemTime,
}

#[async_trait]
pub trait GameManager: Send + Sync {
    async fn create_game(&self, game_name: String) -> anyhow::Result<GameMetadata>;
}
