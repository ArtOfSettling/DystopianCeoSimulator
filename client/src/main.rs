mod systems;

use crate::systems::{
    process_poll_connection_state, process_server_events, send_client_commands,
    setup_connection_resources, setup_pending_player_action, setup_world_state,
};
use bevy::MinimalPlugins;
use bevy::app::{App, FixedUpdate, PluginGroup, ScheduleRunnerPlugin, Startup};
use bevy::prelude::{Fixed, IntoSystemConfigs, Time};
use clap::Parser;
use input_crossterm::CrossTermInputPlugins;
use renderer_ratatui::RatatuiRendererPlugin;
use shared::{ClientArgs, OperatorMode, OperatorModeResource};
use std::time::Duration;
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;

fn main() -> anyhow::Result<()> {
    let args = ClientArgs::parse();
    let operator_mode: OperatorMode = args.operator_mode.into();

    let _guard = setup_logging();
    info!("Logging configured");

    App::new()
        .insert_resource(OperatorModeResource {
            operator_mode: operator_mode.clone(),
        })
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(10))))
        .insert_resource(Time::<Fixed>::from_hz(128.0))
        .add_systems(
            Startup,
            (
                setup_pending_player_action,
                setup_connection_resources,
                setup_world_state,
            )
                .chain(),
        )
        .add_systems(FixedUpdate, process_server_events)
        .add_systems(FixedUpdate, send_client_commands)
        .add_systems(FixedUpdate, process_poll_connection_state)
        .add_plugins(RatatuiRendererPlugin {
            operator_mode: operator_mode.clone(),
        })
        .add_plugins(CrossTermInputPlugins {})
        .run();

    Ok(())
}

fn setup_logging() -> WorkerGuard {
    use tracing_appender::non_blocking;
    use tracing_subscriber::EnvFilter;

    let file_appender = tracing_appender::rolling::daily("_out/logs", "client.log");
    let (non_blocking, guard) = non_blocking(file_appender);
    let env_filter = EnvFilter::new("info");

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();

    guard
}
