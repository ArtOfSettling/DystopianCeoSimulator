use crate::systems::{EventLog, FanOutLogEventReceiver};
use bevy::prelude::ResMut;
use serde::{Deserialize, Serialize};
use shared::InternalEvent;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggedEvent {
    pub version: u32,
    pub timestamp_epoch_millis: u64,
    pub event: InternalEvent,
}

pub fn process_event_log(
    mut log_file: ResMut<EventLog>,
    fan_out_log_event_receiver: ResMut<FanOutLogEventReceiver>,
) {
    while let Ok(logged) = fan_out_log_event_receiver.rx_fan_out_log_events.try_recv() {
        match serde_json::to_string(&logged) {
            Ok(serialized) => {
                if let Err(e) = writeln!(log_file.writer, "{}", serialized) {
                    eprintln!("Failed to write to log file: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to serialize LoggedCommand: {}", e);
            }
        }

        if let Err(e) = log_file.writer.flush() {
            eprintln!("Failed to flush log file: {}", e);
        }
    }
}
