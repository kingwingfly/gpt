use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ModelVersion {
    #[default]
    #[serde(rename = "gpt-4o")]
    GPT4o,
    #[serde(rename = "gpt-4-turbo")]
    GPT4Turbo,
    #[serde(rename = "Meta-Llama-3.1-405B-Instruct")]
    Llama405B,
    #[serde(rename = "Meta-Llama-3.1-70B-Instruct")]
    Llama70B,
    #[serde(rename = "Meta-Llama-3.1-8B-Instruct")]
    Llama8B,
}

impl fmt::Display for ModelVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelVersion::GPT4o => write!(f, "GPT-4o"),
            ModelVersion::GPT4Turbo => write!(f, "GPT-4 Turbo"),
            ModelVersion::Llama405B => write!(f, "Meta-Llama-3.1-405B-Instruct"),
            ModelVersion::Llama70B => write!(f, "Meta-Llama-3.1-70B-Instruct"),
            ModelVersion::Llama8B => write!(f, "Meta-Llama-3.1-8B-Instruct"),
        }
    }
}
