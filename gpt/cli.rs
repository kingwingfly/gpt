#[tokio::main]
async fn main() {
    if let Err(e) = gpt_ui::Cli::run().await {
        eprintln!("Error: {}", e);
    }
}
