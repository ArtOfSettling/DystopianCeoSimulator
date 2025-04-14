mod internal_commands;
mod systems;

use crate::systems::{
    process_client_commands, process_internal_commands, send_server_commands,
    setup_connection_resources, sync_world_state,
};
use bevy::MinimalPlugins;
use bevy::app::{App, FixedUpdate, PluginGroup, ScheduleRunnerPlugin, Startup};
use bevy::time::{Fixed, Time};
use std::time::Duration;
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;

fn main() -> anyhow::Result<()> {
    let _guard = setup_logging();
    info!("Logging configured");

    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(10))))
        .insert_resource(Time::<Fixed>::from_hz(128.0))
        .add_systems(Startup, setup_connection_resources)
        .add_systems(
            FixedUpdate,
            (
                process_internal_commands,
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
