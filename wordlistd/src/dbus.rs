use std::error::Error;

use crate::servers::Request;
use async_trait::async_trait;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use zbus::{connection, fdo, interface, Connection};

struct DBusServer {
    tx: UnboundedSender<Request>,
}

#[interface(
    name = "com.hattmo.Wordlistd1",
    proxy(
        gen_blocking = false,
        default_path = "/com/hattmo/Wordlistd",
        default_service = "com.hattmo.Wordlistd",
    )
)]
impl DBusServer {
    async fn add_word(&self, word: String, tags: Vec<String>) -> Result<(), fdo::Error> {
        self.tx
            .send(server::Request::AddWord(server::AddWord { tags, word }))
            .or(Err(fdo::Error::Failed("Failed to add word".to_string())))
    }
    async fn get_words(&self, tags: Vec<String>) -> Result<Vec<String>, fdo::Error> {
        let (chan, res) = tokio::sync::oneshot::channel();
        self.tx
            .send(server::Request::GetWords(server::GetWords { tags, chan }))
            .or(Err(fdo::Error::Failed("Failed to get words".to_string())))?;
        Ok(res
            .await
            .or(Err(fdo::Error::Failed("Failed to get words".to_string())))?)
    }
}

impl DBusServer {
    fn new(tx: UnboundedSender<server::Request>) -> Self {
        Self { tx }
    }
}

pub struct DbusServerInterface {
    rx: UnboundedReceiver<server::Request>,
    conn: Connection,
}

#[async_trait]
impl ServerInterface for DbusServerInterface {
    async fn get_request(&mut self) -> Option<server::Request> {
        self.rx.recv().await
    }
}
impl DbusServerInterface {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let (tx, rx) = unbounded_channel();
        let conn = connection::Builder::session()?
            .name("com.hattmo.Wordlistd")?
            .serve_at("/com/hattmo/Wordlistd", DBusServer::new(tx))?
            .build()
            .await?;
        Ok(DbusServerInterface { rx, conn })
    }
}
