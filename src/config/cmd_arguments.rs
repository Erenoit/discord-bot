use std::path::PathBuf;

use clap::{Parser, ValueHint};

/// Discord bot for playing music in voice channel and lots of other fun stuff
#[derive(Parser)]
#[command(version)]
pub(super) struct CMDArguments {
    /// Custom config file location
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub cfg_file_path:        Option<PathBuf>,
    /// Custom database folder location
    #[arg(short, long, value_hint = ValueHint::DirPath)]
    pub database_folder_path: Option<PathBuf>,
}
