use snafu::Snafu;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("IO error: {}", source), context(false))]
    Io { source: std::io::Error },

    #[snafu(display("JSON error: {}", source), context(false))]
    Json { source: serde_json::Error },
}
