mod error;

use clap::{Parser, Subcommand};
use error::Result;
use gpt_core::config::Config;

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct App {
    #[command(subcommand)]
    subcmd: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Configure the CLI, showing the current configuration if no arguments are provided.
    Config {
        /// The API endpoint to use. Default: https://api.openai.com/v1/chat/completions
        #[clap(short, long)]
        endpoint: Option<String>,
        /// The API key to use. Stored in the system credential manager.
        #[clap(short, long)]
        api_key: Option<String>,
    },
}

impl App {
    pub fn run() -> Result<()> {
        let cli = Self::parse();
        match cli.subcmd {
            Some(subcmd) => match subcmd {
                Commands::Config { endpoint, api_key } => {
                    if endpoint.is_some() | api_key.is_some() {
                        let mut config = Config::read()?;
                        if let Some(endpoint) = endpoint {
                            config.set_endpoint(endpoint);
                        }
                        if let Some(api_key) = api_key {
                            config.set_api_key(api_key);
                        }
                        config.save()?;
                    }
                    #[cfg(not(feature = "mock"))]
                    let config = Config::read_mask_api_key()?;
                    #[cfg(feature = "mock")]
                    let config = Config::read()?;
                    println!(
                        "endpoint:\t {}\napi_key:\t {}",
                        config.endpoint(),
                        config.api_key()
                    )
                }
            },
            None => {
                println!("No command.");
            }
        }
        Ok(())
    }
}
