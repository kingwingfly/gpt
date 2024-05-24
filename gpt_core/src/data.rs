use serde::{Deserialize, Serialize};

use crate::msg::Role;

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Chunk {
    model: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Choice {
    delta: Delta,
    finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Delta {
    role: Option<Role>,
    content: Option<String>,
}

impl Chunk {
    pub fn new(content: impl AsRef<str>) -> Self {
        Self {
            model: "gpt-4o".to_owned(),
            choices: vec![Choice {
                delta: Delta {
                    role: Some(Role::Assistant),
                    content: Some(content.as_ref().to_owned()),
                },
                finish_reason: None,
            }],
        }
    }

    pub fn content(&self) -> Option<String> {
        self.choices.first().and_then(|c| c.delta.content.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_new() {
        let chunk = Chunk::new("Hello, world!");
        assert_eq!(chunk.content(), Some("Hello, world!".to_owned()));
        let json = r#"{"id":"chatcmpl-9SBD1kHqcT69qO9HIY0aoG6vZOGVL","object":"chat.completion.chunk","created":1716503299,"model":"gpt-4o-2024-05-13","system_fingerprint":"fp_729ea513f7","choices":[{"index":0,"delta":{"role":"assistant","content":"Hello, world!"},"logprobs":null,"finish_reason":null}]}"#;
        let chunk: Chunk = serde_json::from_str(json).unwrap();
        assert_eq!(chunk.content(), Some("Hello, world!".to_owned()));
    }
}
