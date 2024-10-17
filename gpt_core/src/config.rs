use encrypt_config::PersistSource;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::path::PathBuf;
use url::Url;

use crate::model::ModelVersion;

/// The name of the application for config save.
const NAME: &str = "chatGPT";

/// The configuration for the OpenAI chatGPT API,
/// which imlpements the `PersistSource` trait.
/// `load` and `store` is provided.
#[derive(Debug, Serialize, Deserialize, PersistSource)]
#[source(name = "gpt_cli/api_key.json")]
pub struct Config {
    pub endpoint: Url,
    pub api_key: String,
    pub model: ModelVersion,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Use Config:\t{}\ndefault_model:\t{}\nEndpoint:\t{}\nAPI Key:\t{}",
            Config::path().to_string_lossy(),
            self.model,
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
            model: ModelVersion::GPT4o,
        }
    }
}

impl Config {
    pub fn new(endpoint: impl AsRef<str>, api_key: impl AsRef<str>, model: ModelVersion) -> Self {
        Self {
            endpoint: Url::parse(endpoint.as_ref()).unwrap(),
            api_key: api_key.as_ref().to_owned(),
            model,
        }
    }

    pub fn load() -> std::io::Result<Self> {
        <Self as PersistSource>::load()
    }

    pub fn store(&self) -> std::io::Result<()> {
        <Self as PersistSource>::store(self)
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
