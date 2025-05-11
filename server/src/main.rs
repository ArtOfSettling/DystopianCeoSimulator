mod internal_commands;
mod systems;

use crate::systems::{process_client_commands, process_fan_out_commands, process_internal_commands, process_log_commands, send_server_commands, setup_command_log, setup_command_log_replay, setup_connection_resources, setup_fanout_system, setup_world_state, sync_world_state};
use bevy::MinimalPlugins;
use bevy::app::{App, FixedUpdate, PluginGroup, ScheduleRunnerPlugin, Startup};
use bevy::prelude::{IntoSystemConfigs, SystemSet};
use bevy::time::{Fixed, Time};
use std::time::Duration;
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InitSystems;

fn main() -> anyhow::Result<()> {
    let _guard = setup_logging();
    info!("Logging configured");

    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(10))))
        .insert_resource(Time::<Fixed>::from_hz(128.0))
        .add_systems(
            Startup,
            (
                setup_connection_resources,
                setup_world_state,
                setup_fanout_system,
                setup_command_log_replay,
                setup_command_log,
            )
                .chain(),
        )
        .add_systems(
            FixedUpdate,
            (
                process_fan_out_commands,
                process_internal_commands,
                process_log_commands,
                process_client_commands,
                send_server_commands,
                sync_world_state,
            ),
        )
        .run();

    Ok(())
}

fn setup_logging() -> WorkerGuard {
    use tracing_appender::non_blocking;
    use tracing_subscriber::EnvFilter;

    let file_appender = tracing_appender::rolling::daily("_out/logs", "server.log");
    let (non_blocking, guard) = non_blocking(file_appender);
    let env_filter = EnvFilter::new("info");

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();

    guard
}

use bevy::prelude::*;
use std::fs::read_dir;
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Resource)]
pub struct CommandLog {
    pub writer: BufWriter<std::fs::File>,
}

pub fn find_latest_log_file() -> Option<PathBuf> {
    let dir = PathBuf::from("./_out/log_stream");
    let entries = read_dir(&dir).ok()?;

    let mut files: Vec<_> = entries
        .filter_map(Result::ok)
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "ndjson")
                .unwrap_or(false)
        })
        .filter_map(|e| {
            let meta = e.metadata().ok()?;
            let modified = meta.modified().ok()?;
            Some((modified, e.path()))
        })
        .collect();

    files.sort_by_key(|(modified, _)| *modified);
    files.last().map(|(_, path)| path.clone())
}
