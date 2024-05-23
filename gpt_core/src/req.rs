use crate::{
    error::Result,
    model::ModelVersion,
    msg::{Message, Messages, Role},
};
use serde::{Deserialize, Serialize};
use std::{io, path::Path};

#[derive(Debug, Default, Serialize, Deserialize)]
#[non_exhaustive]
struct Req {
    version: ModelVersion,
    messages: Messages,
}

impl Req {
    pub fn new(version: ModelVersion) -> Self {
        Self {
            version,
            ..Default::default()
        }
    }

    pub fn read_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        let res: Req = serde_json::from_reader(reader)?;
        Ok(res)
    }

    pub fn add_message(&mut self, role: Role, content: String) {
        self.messages.push(Message::new(role, content));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OUT_DIR: &str = env!("OUT_DIR");

    #[test]
    fn test_read_from_path() {
        let mut req = Req::new(ModelVersion::GPT4o);
        req.add_message(
            Role::System,
            "You are an experienced Rust programmer.".to_string(),
        );
        req.add_message(Role::User, "How to implemente a http server?".to_string());
        let path = format!("{}/req_test.json", OUT_DIR);
        let file = std::fs::File::create(&path).unwrap();
        serde_json::to_writer(file, &req).unwrap();
        let req = Req::read_from_path(&path).unwrap();
        assert_eq!(req.version, ModelVersion::GPT4o);
        assert_eq!(req.messages.len(), 2);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_model_serde() {
        let model = Req::new(ModelVersion::GPT4o);
        let json = serde_json::to_string(&model).unwrap();
        assert_eq!(json, r#"{"version":"gpt-4o","messages":[]}"#);
        let model: Req = serde_json::from_str(&json).unwrap();
        assert_eq!(model.version, ModelVersion::GPT4o);
    }
}
