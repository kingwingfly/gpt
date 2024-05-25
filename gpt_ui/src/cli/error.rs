use snafu::Snafu;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Core error: {}", source), context(false))]
    Core { source: gpt_core::error::Error },
    #[snafu(display("Dialog error: {}", source), context(false))]
    Dialog { source: dialoguer::Error },
}
