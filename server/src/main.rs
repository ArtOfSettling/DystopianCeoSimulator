mod cli;
mod deterministic_randomization;
mod internal_commands;
mod plugins;
mod systems;

use crate::systems::{
    process_broadcast_world_state, process_client_commands, process_command_log, process_event_log,
    process_fan_out_commands, process_fan_out_events, process_internal_commands,
    process_internal_events, setup_command_log, setup_connection_resources, setup_event_log,
    setup_fan_out_commands, setup_fan_out_events, setup_redrive_command_log,
    setup_redrive_event_log, setup_world_state,
};
use bevy::MinimalPlugins;
use bevy::app::{App, FixedUpdate, PluginGroup, ScheduleRunnerPlugin, Startup};
use bevy::prelude::IntoSystemConfigs;
use bevy::time::{Fixed, Time};
use cli::Cli;
use std::time::Duration;
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;

#[derive(Default, Resource)]
pub struct NeedsWorldBroadcast(pub bool);

#[derive(Default, Resource)]
pub struct NeedsStateUpdate(pub bool);

#[derive(Resource)]
pub struct InternalEventSender {
    pub(crate) tx_internal_events: Sender<InternalEvent>,
}

#[derive(Resource)]
pub struct InternalEventReceiver {
    pub(crate) rx_internal_events: Receiver<InternalEvent>,
}

fn main() -> anyhow::Result<()> {
    let _guard = setup_logging();
    info!("Logging configured");

    let cli = Cli::parse();
    info!("Cli: {:?}", cli);

    let (tx_internal_events, rx_internal_events) = unbounded();

    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(10))))
        .add_plugins(AsyncStdReadySignalPlugin { port: 5555 })
        .insert_resource(Time::<Fixed>::from_hz(128.0))
        .insert_resource(ServerGameState::default())
        .insert_resource(ServerHistoryState::default())
        .insert_resource(NeedsWorldBroadcast::default())
        .insert_resource(NeedsStateUpdate::default())
        .insert_resource(InternalEventSender { tx_internal_events })
        .insert_resource(InternalEventReceiver { rx_internal_events })
        .add_systems(Startup, build_setup_chain(&cli))
        .add_systems(
            FixedUpdate,
            (
                // Process Commands
                process_fan_out_commands,
                process_internal_commands,
                process_command_log,
                // Core gameplay loop
                process_client_commands,
                process_organization_updates,
                process_company_updates,
                // clear any state update flags
                process_clear_needs_state_update,
                // Fan out just prior to broadcasting, so we have the opportunity to save.
                process_fan_out_events,
                process_internal_events,
                process_event_log,
                // Broadcast the new state now that everything is done.
                process_broadcast_world_state,
            )
                .chain(),
        )
        .run();

    Ok(())
}

fn build_setup_chain(cli: &Cli) -> SystemConfigs {
    let base = (
        setup_connection_resources,
        setup_world_state,
        setup_fan_out_commands,
        setup_fan_out_events,
    )
        .chain();

    let with_redrive_cmds = if cli.redrive_command_log {
        (base, setup_redrive_command_log).chain()
    } else {
        base
    };

    let with_redrive_events = if cli.redrive_event_log {
        (with_redrive_cmds, setup_redrive_event_log).chain()
    } else {
        with_redrive_cmds
    };

    (with_redrive_events, setup_command_log, setup_event_log).chain()
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

use crate::plugins::AsyncStdReadySignalPlugin;
use crate::systems::process_clear_needs_state_update::process_clear_needs_state_update;
use crate::systems::process_company_updates::process_company_updates;
use crate::systems::process_organization_updates::process_organization_updates;
use async_channel::{Receiver, Sender, unbounded};
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use clap::Parser;
use shared::{InternalEvent, ServerGameState, ServerHistoryState};
use std::fs::read_dir;
use std::path::PathBuf;

pub fn find_latest_log_file_in_folder(folder: &str) -> Option<PathBuf> {
    let dir = PathBuf::from(folder);
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
