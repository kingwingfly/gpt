mod chat;
mod config;
mod dialog;
mod error;

use clap::{Parser, Subcommand};
use dialog::init_dialog;
use error::Result;

/// The CLI application for interacting with the OpenAI chatGPT API
#[derive(Debug, Parser)]
#[clap(author, version = crate::VERSION, about)]
pub struct Cli {
    #[command(subcommand)]
    subcmd: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Configure the CLI.
    Config,
}

impl Cli {
    pub async fn run() -> Result<()> {
        init_dialog();
        let cli = Self::parse();
        match cli.subcmd {
            Some(subcmd) => match subcmd {
                Commands::Config => config::config(),
            },
            None => chat::chat().await,
        }
    }
}
