use crate::cli::Cli;
use crate::systems::{LoggedEvent, apply_event};
use crate::{Instance, find_latest_log_file_in_folder};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tracing::info;
use uuid::Uuid;

pub fn redrive_event_logs(instance: &mut Instance, game_id: Uuid) {
    let cli = Cli::parse();
    if !cli.redrive_event_log {
        return;
    }

    let Some(log_path) =
        find_latest_log_file_in_folder(format!("./_out/games/{:?}/event_stream", game_id).as_str())
    else {
        info!("No valid event log file found.");
        return;
    };

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
            apply_event(&logged.event, instance);
        }
    }

    info!("Redrive Event Log Complete.");
}
