use crate::{
    config::Config,
    data::Chunk,
    error::Result,
    model::ModelVersion,
    msg::{Message, Messages, Role},
};
use futures_util::StreamExt;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::{io, path::Path};
use std::{path::PathBuf, sync::OnceLock};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Chat {
    #[serde(skip)]
    id: Uuid,
    #[serde(skip)]
    topic: String,
    stream: bool,
    model: ModelVersion,
    messages: Messages,
}

impl Chat {
    pub fn new() -> Self {
        Self {
            id: Uuid::now_v7(),
            stream: true,
            ..Default::default()
        }
    }

    pub fn read_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        let res: Chat = serde_json::from_reader(reader)?;
        Ok(res)
    }

    pub fn save_to_dir<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf> {
        let path = path
            .as_ref()
            .to_path_buf()
            .join(format!("{}.json", self.id));
        let file = std::fs::File::create(&path)?;
        serde_json::to_writer(file, self)?;
        Ok(path)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn stream(&self) -> bool {
        self.stream
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn set_topic(&mut self, topic: String) {
        self.topic = topic;
    }

    pub fn messages(&self) -> &Messages {
        &self.messages
    }

    pub fn add_message(&mut self, role: Role, content: String) {
        self.messages.push(Message::new(role, content));
    }

    pub fn model(&self) -> ModelVersion {
        self.model
    }

    pub fn set_model(&mut self, model: ModelVersion) {
        self.model = model;
    }

    pub fn client() -> &'static Client {
        static CLIENT: OnceLock<Client> = OnceLock::new();
        CLIENT.get_or_init(Client::new)
    }

    pub async fn ask(&self, config: Config, output: &mut impl io::Write) -> Result<()> {
        let mut stream = Self::client()
            .post(config.endpoint().clone())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", config.api_key()),
            )
            .json(&self)
            .send()
            .await?
            .bytes_stream();
        while let Some(item) = stream.next().await {
            let item = item?;
            let chunk = std::str::from_utf8(&item).expect("Invalid UTF-8 sequence");
            for chunk in chunk.split("\n\n") {
                if let Some(chunk) = chunk.strip_prefix("data: ") {
                    if chunk == "[DONE]" {
                        break;
                    }
                    if let Some(chunk) = serde_json::from_str::<Chunk>(chunk)?.content() {
                        output.write_all(chunk.as_bytes())?;
                        output.flush()?;
                    }
                }
            }
        }
        output.write_all(b"\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OUT_DIR: &str = env!("OUT_DIR");

    #[test]
    fn test_read_from_path() {
        let mut chat = Chat::new();
        chat.add_message(
            Role::System,
            "You are an experienced Rust programmer.".to_string(),
        );
        chat.add_message(Role::User, "How to implemente a http server?".to_string());
        let path = chat.save_to_dir(OUT_DIR).unwrap();
        let chat = Chat::read_from_path(&path).unwrap();
        assert_eq!(chat.model, ModelVersion::GPT4o);
        assert_eq!(chat.messages.len(), 2);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_serde() {
        let model = Chat::new();
        let json = serde_json::to_string(&model).unwrap();
        assert_eq!(json, r#"{"stream":true,"model":"gpt-4o","messages":[]}"#);
        let model: Chat = serde_json::from_str(&json).unwrap();
        assert_eq!(model.model, ModelVersion::GPT4o);
    }

    #[cfg(feature = "mock")]
    #[tokio::test]
    async fn mock_chat_ask() {
        use crate::mock::Mock;
        use std::time::Duration;

        let chat = Chat::new();
        let config = Config::new("http://127.0.0.1:3000", "api-key");
        let jh = std::thread::spawn(|| {
            let mock = Mock::new();
            mock.run(3000, Duration::from_secs(1)).unwrap();
        });
        let mut output = vec![];
        chat.ask(config, &mut output).await.unwrap();
        jh.join().unwrap();
    }
}
