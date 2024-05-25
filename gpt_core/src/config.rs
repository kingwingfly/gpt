use crate::error::Result;
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use url::Url;

const NAME: &str = "chatGPT";
const KEYRING_ERROR_HINT: &str = "Keyring Error. Maybe no password manager is installed.";
const KEY_ERROR_HINT: &str = "Invalid key.";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    endpoint: Url,
    #[serde(serialize_with = "serialize")]
    api_key: String,
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
        Ok(())
    }

    pub fn read() -> Result<Self> {
        let file = config_file(false)?;
        let mut config: Config = serde_json::from_reader(file)?;
        config.api_key = keyring_entry().get_password().unwrap();
        Ok(config)
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
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

fn serialize<S>(api_key: &str, serializer: S) -> core::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let entry = keyring_entry();
    entry.set_password(api_key).expect(KEY_ERROR_HINT);
    serializer.serialize_str("stored with keyring")
}

fn keyring_entry() -> &'static Entry {
    use std::sync::OnceLock;
    #[cfg(feature = "mock")]
    keyring::set_default_credential_builder(keyring::mock::default_credential_builder());
    let user = std::env::var("USER").unwrap_or("unknown".to_string());
    static ENTRY: OnceLock<Entry> = OnceLock::new();
    ENTRY.get_or_init(|| Entry::new(NAME, &user).expect(KEYRING_ERROR_HINT))
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
