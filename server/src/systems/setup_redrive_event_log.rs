use crate::systems::{LoggedEvent, apply_event};
use crate::{NeedsWorldBroadcast, find_latest_log_file_in_folder};
use bevy::prelude::ResMut;
use shared::{ServerGameState, ServerHistoryState};
use std::fs::File;
use std::io::{BufRead, BufReader};
use tracing::info;

pub fn setup_redrive_event_log(
    mut server_game_state: ResMut<ServerGameState>,
    mut server_history_state: ResMut<ServerHistoryState>,
    mut needs_world_broadcast: ResMut<NeedsWorldBroadcast>,
) {
    let Some(log_path) = find_latest_log_file_in_folder("./_out/event_stream") else {
        info!("No valid event log file found.");
        return;
    };

    info!("Replaying event log file: {:?}", log_path);

    let file = match File::open(&log_path) {
        Ok(f) => f,
        Err(e) => {
            info!("Failed to open event log file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
        if let Ok(logged) = serde_json::from_str::<LoggedEvent>(&line) {
            info!("Replaying event: {:?}", logged);
            apply_event(
                &logged.event,
                &mut server_game_state,
                &mut server_history_state,
                &mut needs_world_broadcast,
            );
        }
    }

    info!("Redrive Event Log Complete.");
}
