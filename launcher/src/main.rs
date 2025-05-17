mod server_guard;

use crate::server_guard::ServerGuard;
use std::process::Stdio;
use tokio::net::TcpStream;
use tokio::process::{Child, Command};
use tokio::time::{Duration, sleep, timeout};
use tracing::{error, info};
use tracing_appender::non_blocking::WorkerGuard;

async fn is_server_running(port: u16) -> bool {
    TcpStream::connect(("127.0.0.1", port)).await.is_ok()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = setup_logging();
    info!("Launcher starting...");

    let port = 5555;
    let mut server_guard = if is_server_running(port).await {
        info!("Existing server detected, skipping spawn.");
        ServerGuard::external()
    } else {
        info!("No running server detected, spawning new server process.");
        ServerGuard::new("SERVER_PATH", "target/debug/server").await?
    };

    if !wait_for_server_ready(port, 50).await {
        error!("Server failed to become ready in time.");
        if let Err(e) = server_guard.shutdown().await {
            error!("Failed to shutdown server: {}", e);
            return Err(e);
        }
        return Ok(());
    }

    info!("Server is ready. Launching client...");

    let mut client = spawn_foreground_process("CLIENT_PATH", "target/debug/client").await?;

    tokio::select! {
        status = client.wait() => {
            match status {
                Ok(exit) => info!("Client exited with: {}", exit),
                Err(e) => error!("Client process wait failed: {}", e),
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl+C");
        }
    }

    info!("Shutting down server...");
    if let Err(e) = server_guard.shutdown().await {
        error!("Failed to shutdown server: {}", e);
        return Err(e);
    }
    info!("Launcher exiting.");
    Ok(())
}

async fn wait_for_server_ready(port: u16, max_retries: u32) -> bool {
    for _ in 0..max_retries {
        if timeout(
            Duration::from_secs(1),
            TcpStream::connect(("127.0.0.1", port)),
        )
        .await
        .is_ok()
        {
            return true;
        }
        sleep(Duration::from_millis(200)).await;
    }
    false
}

async fn spawn_foreground_process(env_var: &str, default_path: &str) -> anyhow::Result<Child> {
    let path = std::env::var(env_var).unwrap_or_else(|_| default_path.to_string());

    info!("Spawning foreground process: {}", path);
    let child = Command::new(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to spawn foreground process: {}", e))?;

    Ok(child)
}

fn setup_logging() -> WorkerGuard {
    use tracing_appender::non_blocking;
    use tracing_subscriber::EnvFilter;

    let file_appender = tracing_appender::rolling::daily("_out/logs", "launcher.log");
    let (non_blocking, guard) = non_blocking(file_appender);
    let env_filter = EnvFilter::new("debug");

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();

    guard
}
