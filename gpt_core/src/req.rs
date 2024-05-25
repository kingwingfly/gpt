use crate::{chat::Chat, model::ModelVersion, msg::Messages};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Req<'r> {
    stream: bool,
    model: ModelVersion,
    messages: &'r Messages,
}

impl<'r> Req<'r> {
    pub fn new(chat: &'r Chat) -> Self {
        Self {
            stream: chat.stream(),
            model: chat.model(),
            messages: chat.messages(),
        }
    }
}
