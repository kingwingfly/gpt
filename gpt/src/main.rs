fn main() {
    #[cfg(any(feature = "cli", feature = "tui"))]
    let app = gpt_ui::App::run();
}
