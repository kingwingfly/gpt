use reqwest::Url;

pub(crate) struct Config {
    endpoint: Url,
    api_key: String,
}

impl Config {
    pub(crate) fn new(endpoint: impl AsRef<str>, api_key: impl AsRef<str>) -> Self {
        Self {
            endpoint: Url::parse(endpoint.as_ref()).unwrap(),
            api_key: api_key.as_ref().to_owned(),
        }
    }

    pub(crate) fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub(crate) fn api_key(&self) -> &str {
        &self.api_key
    }
}
