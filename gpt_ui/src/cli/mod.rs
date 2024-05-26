mod chat;
mod config;
mod dialog;
mod error;

use clap::{Parser, Subcommand};
use dialog::init_dialog;
use error::Result;
use gpt_core::chat::Chat;

/// The CLI application for interacting with the OpenAI chatGPT API
#[derive(Debug, Parser)]
#[clap(author, version = crate::VERSION, about)]
pub struct Cli {
    #[command(subcommand)]
    subcmd: Option<Commands>,
}

#[derive(Debug, Subcommand)]
#[non_exhaustive]
enum Commands {
    /// Chat with the OpenAI chatGPT API.
    New,
    /// History of chat with the OpenAI chatGPT API.
    History,
    /// Configure the CLI.
    Config,
}

impl Cli {
    pub async fn run() -> Result<()> {
        init_dialog();
        let cli = Self::parse();
        match cli.subcmd {
            Some(subcmd) => match subcmd {
                Commands::New => chat::new_chat(Chat::new()).await,
                Commands::History => chat::history().await,
                Commands::Config => config::config(),
            },
            None => chat::chat().await,
        }
    }
}
