use encrypt_config::PersistSource;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::path::PathBuf;
use url::Url;

/// The name of the application for config save.
const NAME: &str = "chatGPT";

#[derive(Debug, Serialize, Deserialize, PersistSource)]
#[source(name = "gpt_cli/api_key.json")]
pub struct Config {
    pub endpoint: Url,
    pub api_key: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Use Config:\t{}\nEndpoint:\t{}\nAPI Key:\t{}",
            Config::path().to_string_lossy(),
            self.endpoint,
            self.api_key
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            endpoint: Url::parse("https://api.openai.com/v1/chat/completions").unwrap(),
            api_key: String::new(),
        }
    }
}

/// When enable feature `mock`, it uses a mock keyring entry.
/// When not, it uses os' keyring entry.
/// This avoids entering password during tests.
/// If no password manager keyring is able to use, program will panic with error hint.
impl Config {
    pub fn new(endpoint: impl AsRef<str>, api_key: impl AsRef<str>) -> Self {
        Self {
            endpoint: Url::parse(endpoint.as_ref()).unwrap(),
            api_key: api_key.as_ref().to_owned(),
        }
    }
}

pub fn data_dir() -> std::io::Result<PathBuf> {
    #[cfg(feature = "mock")]
    let data_dir = PathBuf::from(env!("OUT_DIR")).join(NAME).join("data");
    #[cfg(not(feature = "mock"))]
    let data_dir = dirs::data_dir()
        .expect("Cannot find data dir.")
        .join(NAME)
        .join("data");
    std::fs::create_dir_all(&data_dir)?;
    Ok(data_dir)
}
