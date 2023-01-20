use std::path::PathBuf;

use clap::{Parser, ValueHint};

/// Discord bot for playing music in voice channel and lots of other fun stuff
#[derive(Parser)]
#[command(version)]
pub(super) struct CMDArguments {
}
