use std::process::Stdio;
use tokio::process::{Child, Command};
use tracing::{error, info};

pub struct ServerGuard {
    child: Option<Child>,
    is_external: bool,
}

impl ServerGuard {
    pub(crate) async fn new(env_var: &str, default_path: &str) -> anyhow::Result<Self> {
        let path = std::env::var(env_var).unwrap_or_else(|_| default_path.to_string());

        info!("Spawning server: {}", path);
        let child = Command::new(path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to spawn server process: {}", e))?;

        Ok(Self {
            child: Some(child),
            is_external: false,
        })
    }

    pub fn external() -> Self {
        Self {
            child: None,
            is_external: true,
        }
    }

    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        if !self.is_external {
            if let Some(child) = &mut self.child {
                if let Some(id) = child.id() {
                    if let Err(e) = child.kill().await {
                        error!("Failed to kill server process with PID {}: {}", id, e);
                    } else {
                        info!("Killed server with PID {}", id);
                    }
                }
            }
        }
        Ok(())
    }
}

impl Drop for ServerGuard {
    fn drop(&mut self) {
        if self.is_external {
            return;
        }
        if let Some(child) = &mut self.child {
            if let Some(id) = child.id() {
                if let Err(e) = std::process::Command::new("kill")
                    .arg("-9")
                    .arg(id.to_string())
                    .status()
                {
                    eprintln!("Failed to kill server with PID {}: {}", id, e);
                } else {
                    eprintln!("Killed server with PID {} (via std::process::Command)", id);
                }
            }
        }
    }
}
