use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(name = "Game Server")]
#[command(about = "Runs the game server loop with optional redrive options", long_about = None)]
#[command(group(
    ArgGroup::new("redrive_mode")
        .args(["redrive_command_log", "redrive_event_log"])
        .multiple(false)
))]
pub struct Cli {
    /// Replays the command log
    #[arg(short = 'c', long = "redrive-command-log", default_value_t = false)]
    pub(crate) redrive_command_log: bool,

    /// Replays the event log
    #[arg(short = 'e', long = "redrive-event-log", default_value_t = true)]
    pub(crate) redrive_event_log: bool,
}
