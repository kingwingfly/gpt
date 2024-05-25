#[cfg(feature = "cli")]
mod cli;
#[cfg(feature = "tui")]
mod tui;

#[cfg(feature = "cli")]
pub use cli::Cli;
#[cfg(feature = "tui")]
pub use tui::Tui;

const VERSION: &str = const_format::formatcp!(
    "{}\nRUSTC: {} {} {}",
    match option_env!("VERGEN_GIT_DESCRIBE") {
        Some(var) => var,
        _ => concat!(env!("CARGO_PKG_VERSION"), "(CARGO_PKG_VERSION)"),
    },
    env!("VERGEN_RUSTC_HOST_TRIPLE"),
    env!("VERGEN_RUSTC_CHANNEL"),
    env!("VERGEN_RUSTC_SEMVER")
);
