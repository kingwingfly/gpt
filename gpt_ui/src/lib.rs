#[cfg(all(feature = "cli", feature = "tui"))]
compile_error!("Only one of the features `cli` and `tui` can be enabled at the same time.");

#[cfg(feature = "cli")]
mod cli;
#[cfg(feature = "tui")]
mod tui;

#[cfg(feature = "cli")]
pub use cli::App;
#[cfg(feature = "tui")]
pub use tui::App;
