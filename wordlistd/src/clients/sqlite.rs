use std::{error::Error, path::Path};

use async_trait::async_trait;
use sqlx::{sqlite::SqliteConnectOptions, Executor, Row, SqlitePool};

use super::Client;

#[derive(Clone)]
pub struct Sqlite {
    pool: SqlitePool,
}

#[async_trait]
impl Client for Sqlite {
    async fn add_word(&mut self, word: &str, tags: &[String]) -> Result<(), Box<dyn Error>> {
        let word_id = self.insert_word_get_id(&word).await?;
        for tag in tags {
            let tag_id = self.insert_tag_get_id(&tag).await?;
            self.insert_word_tag(word_id, tag_id).await?;
        }
        Ok(())
    }
    async fn get_words(&mut self, tags: &[String]) -> Result<Vec<String>, Box<dyn Error>> {
        let filters: Vec<_> = tags
            .iter()
            .map(|tag| format!("tags.tag = \"{tag}\""))
            .collect();
        let filters = filters.join(" AND ");
        let res = self
            .pool
            .fetch_all(sqlx::query(format!("SELECT DISTINCT words.word FROM words JOIN word_tags ON (word_tags.word_id = words.id) JOIN tags ON (tags.id = word_tags.tag_id) WHERE ({filters})").as_str()))
            .await?;
        Ok(res.into_iter().map(|i| i.get::<String, usize>(0)).collect())
    }
}
impl Sqlite {
    pub async fn new(db_path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let db_path = db_path.as_ref();
        let options = SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;
        pool.execute(sqlx::query(
                "CREATE TABLE IF NOT EXISTS words (id INTEGER PRIMARY KEY AUTOINCREMENT,word TEXT UNIQUE)",
            )).await?;
        pool.execute(sqlx::query(
                "CREATE TABLE IF NOT EXISTS tags (id INTEGER PRIMARY KEY AUTOINCREMENT,tag TEXT UNIQUE)",
            )).await?;
        pool.execute(sqlx::query(
                "CREATE TABLE IF NOT EXISTS word_tags (ref_id INTEGER PRIMARY KEY AUTOINCREMENT, word_id INT, tag_id INT, UNIQUE(word_id,tag_id))",
            )).await?;
        Ok(Sqlite { pool })
    }

    async fn insert_word_get_id(&mut self, word: &str) -> Result<i64, Box<dyn Error>> {
        self.pool
            .execute(sqlx::query(
                format!("INSERT OR IGNORE INTO words (word) VALUES ('{word}')").as_str(),
            ))
            .await?;
        let res = self
            .pool
            .fetch_one(sqlx::query(
                format!("SELECT id, word FROM words WHERE word=='{word}'").as_str(),
            ))
            .await?;
        let id: i64 = res.get(0);
        Ok(id)
    }

    async fn insert_tag_get_id(&mut self, tag: &str) -> Result<i64, Box<dyn Error>> {
        self.pool
            .execute(sqlx::query(
                format!("INSERT OR IGNORE INTO tags (tag) VALUES ('{tag}')").as_str(),
            ))
            .await?;
        let res = self
            .pool
            .fetch_one(sqlx::query(
                format!("SELECT id, tag FROM tags WHERE tag=='{tag}'").as_str(),
            ))
            .await?;
        let id: i64 = res.get(0);
        Ok(id)
    }

    async fn insert_word_tag(&mut self, word_id: i64, tag_id: i64) -> Result<(), Box<dyn Error>> {
        self.pool
            .execute(sqlx::query(
                format!(
                    "INSERT OR IGNORE INTO word_tags (word_id,tag_id) VALUES ({word_id},{tag_id})"
                )
                .as_str(),
            ))
            .await?;
        Ok(())
    }
}
