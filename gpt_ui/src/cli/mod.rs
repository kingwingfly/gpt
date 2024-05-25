mod chat;
mod config;
mod error;

use clap::{Parser, Subcommand};
use error::Result;
use gpt_core::config::Config;

/// The CLI application for interacting with the OpenAI chatGPT API
#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct App {
    #[command(subcommand)]
    subcmd: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Configure the CLI.
    Config,
}

impl App {
    pub fn run() -> Result<()> {
        let cli = Self::parse();
        match cli.subcmd {
            Some(subcmd) => match subcmd {
                Commands::Config => config::config(),
            },
            None => chat::chat(),
        }
    }
}
