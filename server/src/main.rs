mod cli;
mod deterministic_randomization;
mod game_management;
mod internal_commands;
mod plugins;
mod systems;

use crate::systems::{
    ClientInfo, create_empty_world_state, process_broadcast_world_state, process_commands,
    process_events, process_internal_commands, process_print_active_connections,
    redrive_event_logs, setup_command_log, setup_event_log, start_server_system,
};
use bevy::MinimalPlugins;
use bevy::app::{App, FixedUpdate, PluginGroup, ScheduleRunnerPlugin, Startup};
use bevy::prelude::IntoSystemConfigs;
use bevy::time::common_conditions::on_timer;
use bevy::time::{Fixed, Time};
use std::collections::HashMap;
use std::time::Duration;
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;

#[derive(Resource)]
pub struct GameServiceResource {
    pub game_service: GameService,
}

#[derive(Resource, Default)]
pub struct Instances {
    pub active_connections: HashMap<Uuid, ClientInfo>,
    pub active_instances: HashMap<Uuid, Instance>,
}

#[derive(Clone)]
pub struct Instance {
    pub instance_game: GameInstanceData,
    pub needs_broadcast: bool,
    pub needs_state_update: bool,
    pub tx_internal_events: Sender<GameClientInternalEvent>,
    pub rx_internal_events: Receiver<GameClientInternalEvent>,
    pub tx_to_clients: Sender<ServerEvent>,
    pub rx_from_client: Receiver<GameClientActionCommand>,
}

#[derive(Clone, Debug)]
pub struct GameClientActionCommand {
    pub source_client_id: Uuid,
    pub game_id: Uuid,
    pub command: ClientActionCommand,
}

#[derive(Clone, Debug)]
pub struct GameClientInternalEvent {
    pub game_id: Uuid,
    pub internal_event: InternalEvent,
}

impl Instances {
    pub fn add_new_instance(
        &mut self,
        game_id: &Uuid,
        tx_to_clients: Sender<ServerEvent>,
        rx_from_clients: Receiver<GameClientActionCommand>,
    ) {
        // send to clients
        // receive from clients
        let (tx_internal_events, rx_internal_events) = unbounded();

        let mut new_instance = Instance {
            instance_game: GameInstanceData {
                game_state: create_empty_world_state(),
                history_state: Default::default(),
            },
            needs_broadcast: false,
            needs_state_update: false,
            tx_internal_events,
            rx_internal_events,
            tx_to_clients,
            rx_from_client: rx_from_clients,
        };
        redrive_event_logs(&mut new_instance, *game_id);
        self.active_instances.insert(*game_id, new_instance);
    }

    pub fn remove_existing_instance(&mut self, instance_id: &Uuid) {
        self.active_instances.remove(instance_id);
    }
}

fn main() -> anyhow::Result<()> {
    let _guard = setup_logging();
    info!("Logging configured");

    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(10))))
        .add_plugins(AsyncStdReadySignalPlugin { port: 5555 })
        .insert_resource(GameServiceResource {
            game_service: GameService::new(Arc::new(FilesystemGameManager::new(
                "./_out/games".into(),
            ))),
        })
        .insert_resource(Time::<Fixed>::from_hz(128.0))
        .insert_resource(Instances {
            active_connections: Default::default(),
            active_instances: Default::default(),
        })
        .add_systems(
            Startup,
            (start_server_system, setup_command_log, setup_event_log).chain(),
        )
        .add_systems(
            Update,
            process_print_active_connections.run_if(on_timer(Duration::from_secs_f32(1.0))),
        )
        .add_systems(
            FixedUpdate,
            (
                // Process Commands
                process_commands,
                process_internal_commands,
                // Core gameplay loop
                process_organization_updates,
                process_company_updates,
                // clear any state update flags
                process_clear_needs_state_update,
                // Fan out just prior to broadcasting, so we have the opportunity to save.
                process_events,
                // Broadcast the new state now that everything is done.
                process_broadcast_world_state,
            )
                .chain(),
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

use crate::game_management::{FilesystemGameManager, GameService};
use crate::plugins::AsyncStdReadySignalPlugin;
use crate::systems::process_clear_needs_state_update::process_clear_needs_state_update;
use crate::systems::process_company_updates::process_company_updates;
use crate::systems::process_organization_updates::process_organization_updates;
use async_channel::{Receiver, Sender, unbounded};
use bevy::prelude::*;
use shared::{ClientActionCommand, GameInstanceData, InternalEvent, ServerEvent};
use std::fs::read_dir;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

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
