use crate::find_latest_log_file_in_folder;
use crate::systems::{FanOutClientCommandSender, LoggedCommand};
use bevy::prelude::Res;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tracing::info;

pub fn setup_redrive_command_log(sender: Res<FanOutClientCommandSender>) {
    let Some(log_path) = find_latest_log_file_in_folder("./_out/command_stream") else {
        info!("No valid command log file found.");
        return;
    };

    info!("Replaying command log file: {:?}", log_path);

    let file = match File::open(&log_path) {
        Ok(f) => f,
        Err(e) => {
            info!("Failed to open command log file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
        if let Ok(logged) = serde_json::from_str::<LoggedCommand>(&line) {
            let _ = sender
                .tx_fan_out_client_action_commands
                .try_send((logged.client_id, logged.command));
        }
    }

    info!("Redrive Command Log Complete.");
}
