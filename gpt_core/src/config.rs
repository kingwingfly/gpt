use crate::error::Result;
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::path::PathBuf;
use url::Url;

/// The name of the application for config save.
const NAME: &str = "chatGPT";
const KEYRING_ERROR_HINT: &str = "Keyring Error. Maybe no password manager is installed.";
const API_KEY_ERROR_HINT: &str = "Failed store API Key. Maybe no password manager is installed.";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    endpoint: Url,
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

    pub fn save(&self) -> Result<()> {
        if !self.api_key.is_empty() {
            keyring_entry()
                .set_password(&serde_json::to_string(self)?)
                .expect(API_KEY_ERROR_HINT);
        }
        Ok(())
    }

    /// Read the config file without masking api_key.
    /// If no Config saved, it will return default Config.
    pub fn read() -> Result<Self> {
        Ok(match keyring_entry().get_password() {
            Ok(serded) => serde_json::from_str(&serded).unwrap_or_default(),
            Err(_) => Self::default(),
        })
    }

    /// Read the config file without reading the api_key.
    /// If no Config saved, it will return default Config.
    pub fn read_masked() -> Result<Self> {
        let mut res = Self::read()?;
        mask(&mut res.api_key);
        Ok(res)
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

fn keyring_entry() -> &'static Entry {
    use std::sync::OnceLock;
    #[cfg(feature = "mock")]
    keyring::set_default_credential_builder(keyring::mock::default_credential_builder());
    let user = std::env::var("USER").unwrap_or("unknown".to_string());
    static ENTRY: OnceLock<Entry> = OnceLock::new();
    ENTRY.get_or_init(|| Entry::new(NAME, &user).expect(KEYRING_ERROR_HINT))
}

fn mask(s: &mut str) {
    let n = std::cmp::min(5, s.len() / 8 + 1);
    let (_, r) = s.split_at_mut(n);
    unsafe {
        let r = r.as_bytes_mut();
        r.fill(b'*');
    }
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
