use crate::systems::LoggedCommand;
use async_channel::{Receiver, Sender, unbounded};
use bevy::prelude::{Commands, Resource};
use shared::ClientActionCommand;
use uuid::Uuid;

#[derive(Resource)]
pub struct FanOutClientCommandSender {
    pub(crate) tx_fan_out_client_action_commands: Sender<(Uuid, ClientActionCommand)>,
}

#[derive(Resource)]
pub struct FanOutClientCommandReceiver {
    pub(crate) rx_fan_out_client_action_commands: Receiver<(Uuid, ClientActionCommand)>,
}

#[derive(Resource)]
pub struct FanOutLogCommandSender {
    pub(crate) tx_fan_out_log_commands: Sender<LoggedCommand>,
}

#[derive(Resource)]
pub struct FanOutLogCommandReceiver {
    pub(crate) rx_fan_out_log_commands: Receiver<LoggedCommand>,
}

pub fn setup_fan_out_commands(mut commands: Commands) {
    let (tx_log_commands, rx_log_commands) = unbounded();
    let (tx_client_commands, rx_client_commands) = unbounded();

    commands.insert_resource(FanOutClientCommandSender {
        tx_fan_out_client_action_commands: tx_client_commands,
    });
    commands.insert_resource(FanOutClientCommandReceiver {
        rx_fan_out_client_action_commands: rx_client_commands,
    });
    commands.insert_resource(FanOutLogCommandSender {
        tx_fan_out_log_commands: tx_log_commands,
    });
    commands.insert_resource(FanOutLogCommandReceiver {
        rx_fan_out_log_commands: rx_log_commands,
    });
}
