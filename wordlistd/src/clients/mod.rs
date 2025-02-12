use async_trait::async_trait;
use std::{error::Error, path::PathBuf};

mod sqlite;
#[derive(Debug)]
pub enum Config {
    Sqllite { path: PathBuf },
}

#[async_trait]
pub trait Client {
    async fn add_word(&mut self, word: &str, tags: &[String]) -> Result<(), Box<dyn Error>>;
    async fn get_words(&mut self, tags: &[String]) -> Result<Vec<String>, Box<dyn Error>>;
}

pub async fn get_client(conf: Config) -> Result<Box<dyn Client>, Box<dyn Error>> {
    match conf {
        Config::Sqllite { path } => Ok(Box::new(sqlite::Sqlite::new(path).await?)),
    }
}
