use crate::find_latest_log_file;
use bevy::prelude::{Commands, Resource};
use chrono::Utc;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Resource)]
pub struct CommandLog {
    pub writer: BufWriter<File>,
}

pub fn setup_command_log(mut commands: Commands) {
    let mut base_path = PathBuf::from("_out/log_stream");
    create_dir_all(&base_path).expect("Failed to create log directory");

    let log_path = find_latest_log_file().unwrap_or_else(|| {
        let timestamp = Utc::now().format("session-%Y%m%d-%H%M%S.ndjson");
        base_path.push(timestamp.to_string());
        base_path
    });

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .expect("Failed to open log file");

    commands.insert_resource(CommandLog {
        writer: BufWriter::new(log_file),
    });

    println!("Using log file: {}", log_path.display());
}
