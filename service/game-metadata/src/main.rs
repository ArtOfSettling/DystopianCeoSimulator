use std::net::SocketAddr;
use std::sync::Arc;
use crate::game_management::GameService;
use crate::game_management::FileSystemGameManager;

mod routes;
mod game_management;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let manager = Arc::new(FileSystemGameManager::new(
        "./_out/games".into(),
    ));
    let service = Arc::new(GameService::new(manager));
    let app = routes::create_router(service);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Game Metadata Service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
