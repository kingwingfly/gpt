use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct App {}

impl App {
    pub fn run() {
        let cli = Self::parse();
        println!("Hello from cli.");
    }
}
