use async_std::net::TcpListener;
use async_std::task;
use bevy::prelude::*;
use std::net::SocketAddr;

#[derive(Resource)]
pub struct Port {
    pub(crate) port: u16,
}

#[derive(Resource)]
#[allow(dead_code)]
pub struct ReadySignalTask(task::JoinHandle<()>);

pub struct AsyncStdReadySignalPlugin {
    pub port: u16,
}

impl Plugin for AsyncStdReadySignalPlugin {
    fn build(&self, app: &mut App) {
        let port = self.port;
        app.add_systems(PostStartup, start_ready_socket);
        app.insert_resource(Port { port });
    }
}

fn start_ready_socket(port_resource: Res<Port>, mut commands: Commands) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port_resource.port));
    info!("Starting async-std TCP ready listener on {}", addr);

    let handle = task::spawn(async move {
        match TcpListener::bind(addr).await {
            Ok(listener) => {
                info!("Ready TCP listener active on {}", addr);
                loop {
                    match listener.accept().await {
                        Ok((_stream, _addr)) => {
                            // Connection accepted and dropped immediately
                        }
                        Err(e) => {
                            error!("Failed to accept connection on ready signal port: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Could not bind to port {}: {:?}", addr, e);
            }
        }
    });

    commands.insert_resource(ReadySignalTask(handle));
}
