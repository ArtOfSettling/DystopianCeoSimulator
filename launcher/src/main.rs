use std::process::Stdio;
use tokio::io::AsyncBufReadExt;
use tokio::process::Command;
use tracing::{error, info};
use tracing_appender::non_blocking::WorkerGuard;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = setup_logging();

    info!("Launcher starting...");

    let server_handle = tokio::spawn(launch_server());

    let client_handle = tokio::spawn(launch_client());

    tokio::select! {
        result = client_handle => {
            match result {
                Ok(status) => match status {
                    Ok(_) => info!("Client exited gracefully. Shutting down launcher."),
                    Err(e) => error!("Client process failed: {:?}", e),
                },
                Err(e) => error!("Client task panicked: {:?}", e),
            }
        }
        server_result = server_handle => {
            if let Err(e) = server_result {
                error!("Server process encountered an error: {:?}", e);
            } else {
                info!("Server process exited unexpectedly.");
            }
        }
    }

    info!("Launcher shutting down.");
    Ok(())
}

async fn launch_server() -> anyhow::Result<()> {
    info!("Starting the server process...");

    let server_path =
        std::env::var("SERVER_PATH").unwrap_or_else(|_| "target/debug/server".to_string());

    let mut server_process = Command::new(server_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start server process");

    if let Some(stdout) = server_process.stdout.take() {
        let mut reader = tokio::io::BufReader::new(stdout).lines();
        tokio::spawn(async move {
            while let Ok(Some(line)) = reader.next_line().await {
                info!("Server: {}", line);
            }
        });
    }

    match server_process.wait().await {
        Ok(status) if status.success() => {
            info!("Server process exited successfully.");
            Ok(())
        }
        Ok(status) => {
            error!("Server process exited with non-zero status: {}", status);
            Err(anyhow::anyhow!("Server process failed"))
        }
        Err(e) => {
            error!("Failed to wait on server process: {:?}", e);
            Err(e.into())
        }
    }
}

async fn launch_client() -> anyhow::Result<()> {
    info!("Starting the client process...");

    let client_path =
        std::env::var("CLIENT_PATH").unwrap_or_else(|_| "target/debug/client".to_string());

    let mut client_process = Command::new(client_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start client process");

    if let Some(stdout) = client_process.stdout.take() {
        let mut reader = tokio::io::BufReader::new(stdout).lines();
        tokio::spawn(async move {
            while let Ok(Some(line)) = reader.next_line().await {
                info!("Client: {}", line);
            }
        });
    }

    match client_process.wait().await {
        Ok(status) if status.success() => {
            info!("Client process exited successfully.");
            Ok(())
        }
        Ok(status) => {
            error!("Client process exited with non-zero status: {}", status);
            Err(anyhow::anyhow!("Client process failed"))
        }
        Err(e) => {
            error!("Failed to wait on client process: {:?}", e);
            Err(e.into())
        }
    }
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
