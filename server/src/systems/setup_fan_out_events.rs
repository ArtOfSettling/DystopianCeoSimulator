use crate::systems::LoggedEvent;
use async_channel::{Receiver, Sender, unbounded};
use bevy::prelude::{Commands, Resource};
use shared::InternalEvent;

#[derive(Resource)]
pub struct FanOutLogEventSender {
    pub(crate) tx_fan_out_log_events: Sender<LoggedEvent>,
}

#[derive(Resource)]
pub struct FanOutLogEventReceiver {
    pub(crate) rx_fan_out_log_events: Receiver<LoggedEvent>,
}

#[derive(Resource)]
pub struct FanOutEventSender {
    pub(crate) tx_fan_out_events: Sender<InternalEvent>,
}

#[derive(Resource)]
pub struct FanOutEventReceiver {
    pub(crate) rx_fan_out_events: Receiver<InternalEvent>,
}

pub fn setup_fan_out_events(mut commands: Commands) {
    let (tx_fan_out_log_events, rx_fan_out_log_events) = unbounded();
    let (tx_fan_out_events, rx_fan_out_events) = unbounded();

    commands.insert_resource(FanOutLogEventSender {
        tx_fan_out_log_events,
    });
    commands.insert_resource(FanOutLogEventReceiver {
        rx_fan_out_log_events,
    });
    commands.insert_resource(FanOutEventSender { tx_fan_out_events });
    commands.insert_resource(FanOutEventReceiver { rx_fan_out_events });
}
