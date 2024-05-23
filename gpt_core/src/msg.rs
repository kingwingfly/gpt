use serde::{Deserialize, Serialize};

pub(crate) type Messages = Vec<Message>;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Message {
    role: Role,
    content: String,
}

impl Message {
    pub(crate) fn new(role: Role, content: String) -> Self {
        Self { role, content }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Role {
    System,
    User,
    Assistant,
}
