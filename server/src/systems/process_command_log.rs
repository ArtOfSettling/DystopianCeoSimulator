use crate::systems::FanOutLogCommandReceiver;
use crate::systems::setup_command_log::CommandLog;
use bevy::prelude::ResMut;
use serde::{Deserialize, Serialize};
use shared::ClientActionCommand;
use std::io::Write;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggedCommand {
    pub timestamp_epoch_millis: u64,
    pub client_id: Uuid,
    pub command: ClientActionCommand,
}

pub fn process_command_log(
    mut log_file: ResMut<CommandLog>,
    fan_out_log_command_receiver: ResMut<FanOutLogCommandReceiver>,
) {
    while let Ok(logged) = fan_out_log_command_receiver
        .rx_fan_out_log_commands
        .try_recv()
    {
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
