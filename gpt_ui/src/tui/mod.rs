mod error;

use error::Result;

pub struct Tui {}

impl Tui {
    pub fn run() -> Result<()> {
        println!("Hello from tui.");
        Ok(())
    }
}
