use crate::find_latest_log_file;
use crate::systems::{FanOutClientCommandSender, LoggedCommand};
use bevy::prelude::Res;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tracing::info;

pub fn setup_command_log_replay(sender: Res<FanOutClientCommandSender>) {
    let Some(log_path) = find_latest_log_file() else {
        info!("No valid log file found.");
        return;
    };

    info!("Replaying log file: {:?}", log_path);

    let file = match File::open(&log_path) {
        Ok(f) => f,
        Err(e) => {
            info!("Failed to open log file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines().flatten() {
        if let Ok(logged) = serde_json::from_str::<LoggedCommand>(&line) {
            let _ = sender
                .tx_fan_out_client_commands
                .try_send((logged.client_id, logged.command));
        }
    }

    info!("Replay complete.");
}
