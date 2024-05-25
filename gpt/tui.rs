fn main() {
    if let Err(e) = gpt_ui::Tui::run() {
        eprintln!("Error: {}", e);
    }
}
