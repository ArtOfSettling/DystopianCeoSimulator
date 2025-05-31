use crate::InternalEventReceiver;
use crate::systems::{FanOutEventSender, FanOutLogEventSender, LoggedEvent};
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
pub fn process_fan_out_events(
    receiver: Res<InternalEventReceiver>,
    fan_out_log_tx: ResMut<FanOutLogEventSender>,
    fan_out_tx: ResMut<FanOutEventSender>,
) {
    while let Ok(event) = receiver.rx_internal_events.try_recv() {
        let logged = LoggedEvent {
            version: 1,
            timestamp_epoch_millis: current_millis(),
            event: event.clone(),
        };

        let _ = fan_out_log_tx.tx_fan_out_log_events.try_send(logged);
        let _ = fan_out_tx.tx_fan_out_events.try_send(event.clone());
    }
}
