use crate::find_latest_log_file_in_folder;
use bevy::prelude::{Commands, Resource};
use chrono::Utc;
use std::collections::HashMap;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::BufWriter;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Resource)]
pub struct EventLog {
    pub writer: EventLogWriter,
}

pub struct EventLogWriter {
    pub writers: HashMap<Uuid, BufWriter<File>>,
}

impl EventLogWriter {
    pub fn get_writer(&mut self, game_id: &Uuid) -> &mut BufWriter<File> {
        if !self.writers.contains_key(game_id) {
            let file = new_event_log(game_id);
            self.writers.insert(*game_id, BufWriter::new(file));
        }
        self.writers.get_mut(game_id).unwrap()
    }
}

pub fn setup_event_log(mut commands: Commands) {
    commands.insert_resource(EventLog {
        writer: EventLogWriter {
            writers: Default::default(),
        },
    });
}

fn new_event_log(game_id: &Uuid) -> File {
    get_event_stream(game_id, "event_stream")
}

pub fn get_event_stream(game_id: &Uuid, stream_loc: &str) -> File {
    let event_log_path = format!("./_out/games/{}/{}", game_id, stream_loc);
    let mut event_log_path_buf = PathBuf::from(event_log_path.as_str());

    create_dir_all(&event_log_path_buf).unwrap_or_else(|e| {
        panic!(
            "Failed to create stream directory {}: {}",
            event_log_path, e
        )
    });

    let log_path = find_latest_log_file_in_folder(event_log_path.as_str()).unwrap_or_else(|| {
        let timestamp = Utc::now().format("session-%Y%m%d-%H%M%S.ndjson");
        event_log_path_buf.push(timestamp.to_string());
        event_log_path_buf
    });

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .expect("Failed to open log file")
}
