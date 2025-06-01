use crate::systems::get_event_stream;
use bevy::prelude::{Commands, Resource};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use uuid::Uuid;

#[derive(Resource)]
pub struct CommandLog {
    pub writer: CommandLogWriter,
}

pub struct CommandLogWriter {
    pub writers: HashMap<Uuid, BufWriter<File>>,
}

impl CommandLogWriter {
    pub fn get_writer(&mut self, game_id: &Uuid) -> &mut BufWriter<File> {
        if !self.writers.contains_key(game_id) {
            let file = new_command_log(game_id);
            self.writers.insert(*game_id, BufWriter::new(file));
        }
        self.writers.get_mut(game_id).unwrap()
    }
}

pub fn setup_command_log(mut commands: Commands) {
    commands.insert_resource(CommandLog {
        writer: CommandLogWriter {
            writers: Default::default(),
        },
    });
}

fn new_command_log(game_id: &Uuid) -> File {
    get_event_stream(game_id, "command_stream")
}
