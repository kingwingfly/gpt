use snafu::Snafu;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("IO error: {}", source), context(false))]
    Io { source: std::io::Error },
    #[snafu(display("JSON error: {}", source), context(false))]
    Json { source: serde_json::Error },
    #[snafu(display("Reqwest error: {}", source), context(false))]
    Reqwest { source: reqwest::Error },
    #[cfg_attr(
        not(feature = "mock"),
        snafu(display("Keyring error: {}", source), context(false))
    )]
    #[cfg_attr(
        feature = "mock",
        snafu(display("**Mock** keyring error: {}", source), context(false))
    )]
    Keyring { source: keyring::error::Error },
}
