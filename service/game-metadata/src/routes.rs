use axum::{
    extract::{Path, State},
    routing::{get, delete},
    Json, Router,
};
use shared::GameMetadata;
use std::sync::Arc;
use uuid::Uuid;
use crate::game_management::GameService;

pub fn create_router(service: Arc<GameService>) -> Router {
    Router::new()
        .route("/games", get(list_games).post(create_game))
        .route("/games/{id}", delete(delete_game))
        .with_state(service)
}

async fn create_game(
    State(service): State<Arc<GameService>>,
    Json(payload): Json<CreateGameRequest>,
) -> Result<Json<GameMetadata>, (axum::http::StatusCode, String)> {
    service.create_game(payload.name)
        .await
        .map(Json)
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))
}

async fn list_games(
    State(service): State<Arc<GameService>>,
) -> Result<Json<Vec<GameMetadata>>, (axum::http::StatusCode, String)> {
    service.list_games()
        .await
        .map(Json)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn delete_game(
    Path(id): Path<Uuid>,
    State(service): State<Arc<GameService>>,
) -> Result<(), (axum::http::StatusCode, String)> {
    service.delete_game(id)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))
}

#[derive(serde::Deserialize)]
pub struct CreateGameRequest {
    pub name: String,
}
