use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) enum ModelVersion {
    #[default]
    #[serde(rename = "gpt-4o")]
    GPT4o,
    #[serde(rename = "gpt-4-turbo")]
    GPT4Turbo,
}
