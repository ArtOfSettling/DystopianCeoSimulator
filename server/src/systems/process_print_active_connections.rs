use crate::systems::ClientInfo;
use crate::{Instance, Instances};
use bevy::prelude::Res;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use uuid::Uuid;

fn render_debug_state(
    instances: &HashMap<Uuid, Instance>,
    connections: &HashMap<Uuid, ClientInfo>,
) {
    // Clear screen and move cursor to top-left
    print!("\x1B[2J\x1B[1;1H");

    let now = chrono::Utc::now();
    println!("=== Server Debug Info @ {} ===", now);
    println!();

    println!("Active Game Instances [{}]:", instances.len());
    for (instance_id, instance) in instances.iter() {
        println!("Instance ID: {}", instance_id);
        println!(" - Needs Broadcast: {}", instance.needs_broadcast);
        println!(" - Needs State Update: {}", instance.needs_state_update);
        println!();
    }

    println!("Connected Clients [{}]:", connections.len());
    for (client_id, conn) in connections.iter() {
        println!("Client ID: {}", client_id);
        println!(" - Game ID: {}", conn.game_id);
        println!(" - Operator Mode: {:?}", conn.operator_mode);
        println!(" - Addr: {}", conn.addr);
        println!();
    }

    io::stdout().flush().unwrap();
}

pub fn process_print_active_connections(instance: Res<Instances>) {
    let instances = instance.active_instances.clone();
    let connections = instance.active_connections.clone();

    bevy::tasks::IoTaskPool::get()
        .spawn(async move {
            render_debug_state(&instances, &connections);
        })
        .detach();
}
