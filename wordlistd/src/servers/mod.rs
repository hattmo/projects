use std::error::Error;

use async_trait::async_trait;

#[derive(Debug)]
pub enum Config {
    Dbus,
}

use tokio::sync::oneshot::Sender;

pub enum Request {
    AddWord(AddWord),
    GetWords(GetWords),
}

pub struct AddWord {
    pub word: String,
    pub tags: Vec<String>,
}

pub struct GetWords {
    pub tags: Vec<String>,
    pub(crate) chan: Sender<Vec<String>>,
}

impl GetWords {
    pub fn reply(self, words: Vec<String>) {
        let _ = self.chan.send(words);
    }
}

#[async_trait]
pub trait Server {
    async fn get_request(&mut self) -> Option<Request>;
}

pub async fn get_server() -> Result<Box<dyn Server>, Box<dyn Error>> {
    return Ok(Box::new(DbusServerInterface::new().await?));
}
