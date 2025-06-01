use crate::game_management::GameManager;
use shared::GameMetadata;
use std::sync::Arc;

#[derive(Clone)]
pub struct GameService {
    manager: Arc<dyn GameManager>,
}

impl GameService {
    pub fn new(manager: Arc<dyn GameManager>) -> Self {
        Self { manager }
    }

    pub async fn create_game(&self, game_name: String) -> anyhow::Result<GameMetadata> {
        if game_name.trim().is_empty() {
            anyhow::bail!("Game name cannot be empty");
        }

        let metadata = self.manager.create_game(game_name).await?;
        Ok(metadata)
    }

    pub async fn list_games(&self) -> anyhow::Result<Vec<GameMetadata>> {
        let mut games = self.manager.list_games().await?;
        games.sort_by_key(|m| std::cmp::Reverse(m.created_at));
        Ok(games)
    }
}
