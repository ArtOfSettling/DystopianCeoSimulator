use crate::game_management::{GameManager, GameMetadata};
use async_trait::async_trait;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use uuid::Uuid;

pub struct FilesystemGameManager {
    base_path: PathBuf,
}

impl FilesystemGameManager {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    fn metadata_path(&self, game_id: Uuid) -> PathBuf {
        self.base_path
            .join(game_id.to_string())
            .join("metadata.json")
    }

    fn game_path(&self, game_id: Uuid) -> PathBuf {
        self.base_path.join(game_id.to_string())
    }
}

#[async_trait]
impl GameManager for FilesystemGameManager {
    async fn create_game(&self, game_name: String) -> anyhow::Result<GameMetadata> {
        let game_id = Uuid::new_v4();
        let game_dir = self.game_path(game_id);
        fs::create_dir_all(&game_dir)?;
        fs::create_dir_all(game_dir.join("command_stream"))?;
        fs::create_dir_all(game_dir.join("event_stream"))?;

        let metadata = GameMetadata {
            id: game_id,
            name: game_name,
            created_at: SystemTime::now(),
        };

        let metadata_file = self.metadata_path(game_id);
        let json = serde_json::to_string_pretty(&metadata)?;
        fs::write(metadata_file, json)?;

        Ok(metadata)
    }
}
