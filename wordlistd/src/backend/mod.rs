use async_trait::async_trait;
use std::io;

mod sqlite;
pub use sqlite::DB as SqliteDB;

pub struct BackendError;

impl From<io::Error> for BackendError {
    fn from(_value: io::Error) -> Self {
        BackendError
    }
}
impl From<sqlx::Error> for BackendError {
    fn from(_value: sqlx::Error) -> Self {
        BackendError
    }
}

#[async_trait]
pub trait Backend {
    async fn add_word(&mut self, word: &str, tags: &[String]) -> Result<(), BackendError>;
    async fn get_words(&mut self, tags: &[String]) -> Result<Vec<String>, BackendError>;
}
