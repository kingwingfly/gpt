use crate::error::Result;
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs::OpenOptions;
use url::Url;

/// The name of the application for config save.
const NAME: &str = "chatGPT";
const KEYRING_ERROR_HINT: &str = "Keyring Error. Maybe no password manager is installed.";
const API_KEY_ERROR_HINT: &str = "Failed store API Key. Maybe no password manager is installed.";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    endpoint: Url,
    #[serde(skip, default = "masked")]
    api_key: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Endpoint:\t {}\nAPI Key:\t {}",
            self.endpoint, self.api_key
        )
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

    pub fn save(&self) -> Result<()> {
        let file = config_file(true)?;
        serde_json::to_writer(file, self)?;
        keyring_entry()
            .set_password(&self.api_key)
            .expect(API_KEY_ERROR_HINT);
        Ok(())
    }

    pub fn read() -> Result<Self> {
        let config = match Self::read_masked() {
            Ok(mut config) => {
                config.api_key = keyring_entry()
                    .get_password()
                    .unwrap_or("unknown".to_string());
                config
            }
            Err(_) => Config::new("https://api.openai.com/v1/chat/completions", "unknown"),
        };
        Ok(config)
    }

    /// Read the config file without reading the api_key.
    pub fn read_masked() -> Result<Self> {
        Ok(serde_json::from_reader(config_file(false)?)?)
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub fn set_endpoint(&mut self, endpoint: impl AsRef<str>) {
        self.endpoint = Url::parse(endpoint.as_ref()).expect("Invalid URL.");
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn set_api_key(&mut self, api_key: impl AsRef<str>) {
        self.api_key = api_key.as_ref().to_string();
    }
}

fn config_file(truncate: bool) -> std::io::Result<std::fs::File> {
    let config_dir = dirs::config_dir()
        .expect("Cannot find config dir.")
        .join(NAME);
    std::fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("config.json");
    OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .truncate(truncate)
        .open(config_path)
}

fn keyring_entry() -> &'static Entry {
    use std::sync::OnceLock;
    #[cfg(feature = "mock")]
    keyring::set_default_credential_builder(keyring::mock::default_credential_builder());
    let user = std::env::var("USER").unwrap_or("unknown".to_string());
    static ENTRY: OnceLock<Entry> = OnceLock::new();
    ENTRY.get_or_init(|| Entry::new(NAME, &user).expect(KEYRING_ERROR_HINT))
}

fn masked() -> String {
    "********".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_save_read() {
        let config = Config::new("http://localhost:8080", "api_key");
        config.save().unwrap();
        let read = Config::read().unwrap();
        assert_eq!(config.endpoint(), read.endpoint());
        assert_eq!(config.api_key(), read.api_key());
    }
}
