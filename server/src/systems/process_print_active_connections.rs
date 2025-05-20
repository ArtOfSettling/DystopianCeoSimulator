use crate::systems::{ActiveConnection, ConnectionResources};
use bevy::prelude::Res;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use tracing::info;
use uuid::Uuid;

fn render_active_connections(map: &HashMap<Uuid, ActiveConnection>) {
    // ANSI escape codes: clear screen and move cursor to top-left
    print!("\x1B[2J\x1B[1;1H");
    println!("Active Connections [{}]:", chrono::Utc::now());
    info!("Active Connections: {}", chrono::Utc::now());

    for (uuid, conn) in map.iter() {
        println!(" - {} @ {:?}", uuid, conn.addr);
        info!(" - {} @ {:?}", uuid, conn.addr)
    }

    io::stdout().flush().unwrap();
}

pub fn process_print_active_connections(res: Res<ConnectionResources>) {
    let active_connections = res.active_connections.clone();
    bevy::tasks::IoTaskPool::get()
        .spawn(async move {
            let map = active_connections.lock().await;
            render_active_connections(&map);
        })
        .detach();
}
