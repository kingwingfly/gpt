use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct App {
    #[command(subcommand)]
    subcmd: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {}

impl App {
    pub fn run() {
        let cli = Self::parse();
        match cli.subcmd {
            Some(_) => {
                unimplemented!()
            }
            None => {
                println!("No command.");
            }
        }
    }
}
