use crate::systems::{
    ClientActionCommandReceiver, FanOutClientCommandSender, FanOutLogCommandSender, LoggedCommand,
};
use bevy::prelude::{Res, ResMut};
use std::time::{SystemTime, UNIX_EPOCH};

fn current_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

// Provides fan-out capabilities. Consumes events via the receiver and fans them out
// to all who need to listen.
pub fn process_fan_out_commands(
    receiver: Res<ClientActionCommandReceiver>,
    log_tx: ResMut<FanOutLogCommandSender>,
    process_tx: ResMut<FanOutClientCommandSender>,
) {
    while let Ok((client_id, command)) = receiver.rx_client_action_commands.try_recv() {
        let logged = LoggedCommand {
            timestamp_epoch_millis: current_millis(),
            client_id,
            command: command.clone(),
        };

        let _ = log_tx.tx_fan_out_log_commands.try_send(logged);
        let _ = process_tx
            .tx_fan_out_client_action_commands
            .try_send((client_id, command));
    }
}
