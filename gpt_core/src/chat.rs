use crate::{
    config::Config,
    data::Chunk,
    error::Result,
    model::ModelVersion,
    msg::{Message, Messages, Role},
    req::Req,
};
use futures_util::StreamExt;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    io,
    path::{Path, PathBuf},
    sync::OnceLock,
};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[non_exhaustive]
pub struct Chat {
    id: Uuid,
    topic: String,
    temperature: f32,
    stream: bool,
    model: ModelVersion,
    messages: Messages,
}

impl Display for Chat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chat ID: {}", self.id)?;
        writeln!(f, "Topic: {}", self.topic)?;
        writeln!(f, "Model: {:?}", self.model)?;
        if !self.messages.is_empty() {
            writeln!(f, "History Dialog:")?;
            for msg in &self.messages {
                writeln!(f, "{}", msg)?;
            }
        }
        Ok(())
    }
}

impl Chat {
    pub fn new() -> Self {
        Self {
            id: Uuid::now_v7(),
            stream: true,
            temperature: 0.3,
            ..Default::default()
        }
    }

    pub fn summary_extraction() -> Self {
        let mut res = Self {
            id: Uuid::now_v7(),
            stream: true,
            temperature: 0.,
            ..Default::default()
        };
        res.add_message(Role::System, "You are a highly skilled AI trained in language comprehension and summarization. I would like you to read the following text and summarize its topic in a pretty short sentence, aiming to be used as an article tittle".to_string());
        res
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
            .join(sanitize_filename::sanitize(format!(
                "{}-{}.json",
                self.topic(),
                self.id
            )));
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

    pub async fn ask(&self, config: &Config, output: &mut impl io::Write) -> Result<String> {
        let mut content = String::new();
        let mut stream = Self::client()
            .post(config.endpoint.clone())
            .header(header::AUTHORIZATION, format!("Bearer {}", config.api_key))
            .json(&self.req())
            .send()
            .await?
            .bytes_stream();
        let mut buffer = String::new();
        'a: while let Some(item) = stream.next().await {
            let item = item?;
            let chunk = std::str::from_utf8(&item).expect("Invalid UTF-8 sequence");
            buffer.push_str(chunk);
            for chunk in buffer.split("\n\n") {
                if let Some(chunk) = chunk.strip_prefix("data: ") {
                    if chunk == "[DONE]" {
                        break;
                    }
                    match serde_json::from_str::<Chunk>(chunk) {
                        Ok(chunk) => {
                            if let Some(chunk) = chunk.content() {
                                content.push_str(&chunk);
                                output.write_all(chunk.as_bytes())?;
                                output.flush()?;
                            }
                        }
                        Err(_) => {
                            buffer = format!("data: {chunk}");
                            continue 'a;
                        }
                    }
                }
            }
            buffer.clear();
        }
        output.write_all(b"\n")?;
        Ok(content)
    }

    pub fn req(&self) -> Req {
        Req::new(self)
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
        let deserialized: Chat = serde_json::from_str(&json).unwrap();
        assert_eq!(model, deserialized);
    }

    #[cfg(feature = "mock")]
    #[tokio::test]
    #[ignore = "GitHub Actions does not support mock server"]
    async fn mock_chat_ask() {
        use crate::mock::Mock;
        use std::time::Duration;

        let chat = Chat::new();
        let config = Config::new("http://127.0.0.1:3000", "api-key", ModelVersion::GPT4o);
        let mock = Mock::new(3000, Duration::from_secs(1));
        let mut output = vec![];
        let content = chat.ask(&config, &mut output).await.unwrap();
        assert_eq!(output, b"Response from mock server.\n");
        assert_eq!(content, "Response from mock server.");
        mock.close();
    }
}
