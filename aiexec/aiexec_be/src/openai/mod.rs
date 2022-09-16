mod db;

use anyhow::Result;
use mongodb::{bson::Uuid, Client as MongoClient};
use reqwest::Client as ReqwestClient;
use serde;
use std::collections::HashMap;

use self::db::get_session_data;
#[derive(serde::Serialize)]
#[serde(untagged)]
enum StringOrArray<'a> {
    String(&'a str),
    Array(&'a [&'a str]),
}

#[derive(serde::Serialize, Default)]

struct OpenAIRequest<'a> {
    model: &'a str,
    messages: &'a [Message],
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,
    #[serde(rename = "n")]
    #[serde(skip_serializing_if = "Option::is_none")]
    num_completions: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<StringOrArray<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logit_bias: Option<&'a HashMap<&'a str, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<&'a str>,
}

//Response from OpenAI
#[derive(serde::Deserialize)]
struct OpenAIResponse {
    id: String,
    object: String,
    created: u64,
    choices: Vec<Choice>,
    useage: Usage,
}

#[derive(serde::Deserialize)]
struct Usage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

#[derive(serde::Deserialize)]
struct Choice {
    index: usize,
    message: Message,
    finish_reason: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

async fn complete_chat(client: ReqwestClient, messages: &mut Vec<Message>) -> Result<()> {
    let mut req = OpenAIRequest::default();
    req.messages = messages;
    req.model = "gpt-3.5-turbo";
    let mut res: OpenAIResponse = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer: {}", crate::ARGS.openai_key),
        )
        .json(&req)
        .send()
        .await?
        .json()
        .await?;
    messages.push(res.choices.remove(0).message);
    todo!()
}

#[derive(Clone)]
pub struct Sessions(MongoClient);

impl Sessions {
    pub async fn new() -> Result<Self> {
        let client = db::create_client().await?;
        Ok(Self(client))
    }
    pub async fn get_session(Self(client): &Self, session_id: Uuid) -> Result<Session> {
        let data = get_session_data(client, session_id).await?;
        Ok(Session {
            client: client.clone(),
            data,
        })
    }
}

pub struct Session {
    client: MongoClient,
    data: db::SessionData,
}

impl Session {
    pub fn get_messages(&self) -> &[Message] {
        self.data.messages.as_slice()
    }
    pub fn add_message(&mut self, message: Message) {
        let messages = &mut self.data.messages;
        messages.push(message);
    }
}
