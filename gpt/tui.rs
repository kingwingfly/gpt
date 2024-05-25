fn main() {
    if let Err(e) = gpt_ui::App::run().await {
        eprintln!("Error: {}", e);
    }
}
