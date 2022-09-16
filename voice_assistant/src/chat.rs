use reqwest::Client;
use serde;
use serde_json;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug)]
struct Chat {
    client: Client,
    chats: Vec<Message>,
}

#[derive(serde::Deserialize)]
struct Completion {
    choices: Vec<Choice>,
}

#[derive(serde::Deserialize)]
struct Choice {
    message: Message,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
struct Message {
    role: String,
    content: String,
}

impl Chat {
    fn new() -> Self {
        Self {
            client: Client::new(),
            chats: vec![Message {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            }],
        }
    }
    async fn chat(&mut self, request: String, key: &str) -> Option<String> {
        self.chats.push(Message {
            role: "user".to_string(),
            content: request,
        });
        let res = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", key)
            .json(&serde_json::json!({
                "model": "gpt-3.5-turbo",
                "messages": self.chats,
            }))
            .send()
            .await
            .ok()?;
        let body: Completion = res.json().await.ok()?;
        self.chats.push(body.choices.get(0)?.message.clone());
        Some(body.choices.get(0)?.message.content.clone())
    }
}

pub struct Chats {
    chats: Mutex<HashMap<String, Chat>>,
    key: String,
}

impl Chats {
    pub fn new() -> Self {
        Self {
            key: format!(
                "Bearer {}",
                std::env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY")
            ),
            chats: Mutex::new(HashMap::new()),
        }
    }
    pub async fn chat(&self, id: String, request: String) -> Option<String> {
        let mut map = self.chats.lock().await;
        let chat = map.entry(id).or_insert_with(Chat::new);
        chat.chat(request, &self.key).await
    }

    pub async fn clear_chat(&self, id: String) {
        self.chats.lock().await.remove(&id);
    }
}
