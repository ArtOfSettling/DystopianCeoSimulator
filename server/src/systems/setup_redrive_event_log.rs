use crate::find_latest_log_file_in_folder;
use crate::systems::{FanOutEventSender, LoggedEvent};
use bevy::prelude::Res;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tracing::info;

pub fn setup_redrive_event_log(sender: Res<FanOutEventSender>) {
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
            let _ = sender.tx_fan_out_events.try_send(logged.event);
        }
    }

    info!("Redrive Event Log Complete.");
}
