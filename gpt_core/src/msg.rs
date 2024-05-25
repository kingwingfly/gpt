use serde::{Deserialize, Serialize};

pub type Messages = Vec<Message>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    role: Role,
    content: String,
}

impl Message {
    pub fn new(role: Role, content: String) -> Self {
        Self { role, content }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}
