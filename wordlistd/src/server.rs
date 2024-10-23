use std::io;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::UnixStream,
};

use crate::{
    backend::{Backend, BackendError},
    util::GetLine,
};

pub enum ServerError {
    Io(io::Error),
    Backend(BackendError),
}

impl From<io::Error> for ServerError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<BackendError> for ServerError {
    fn from(value: BackendError) -> Self {
        Self::Backend(value)
    }
}

struct ClientError;

impl From<io::Error> for ClientError {
    fn from(_value: io::Error) -> Self {
        Self
    }
}

impl From<BackendError> for ClientError {
    fn from(_value: BackendError) -> Self {
        Self
    }
}

pub async fn main() -> Result<(), ServerError> {
    let _ = tokio::fs::remove_file("./sock").await; // ignore missing
    let server = tokio::net::UnixListener::bind("./sock")?;
    let db = Box::new(crate::backend::SqliteDB::new("./test_db").await?);
    while let Ok((client, _)) = server.accept().await {
        tokio::spawn(handle_client(client, db.clone()));
    }
    Ok(())
}

async fn handle_client(client: UnixStream, db: Box<dyn Backend + Send>) -> Result<(), ClientError> {
    let mut client = BufReader::new(client);
    match client.read_line_alloc().await?.as_str() {
        "add" => handle_add(&mut client, db).await,
        "get" => handle_get(&mut client, db).await,
        _ => Err(ClientError),
    }
}

async fn handle_add<T>(
    client: &mut BufReader<T>,
    mut db: Box<dyn Backend + Send>,
) -> Result<(), ClientError>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    let mut tags = Vec::new();
    let word = client.read_line_alloc().await?;
    if word.is_empty() {
        return Err(ClientError);
    }
    loop {
        let next_tag = client.read_line_alloc().await?;
        if next_tag.is_empty() {
            break;
        }
        tags.push(next_tag);
    }
    if tags.is_empty() {
        return Err(ClientError);
    }
    db.add_word(&word, &tags).await?;
    Ok(())
}
async fn handle_get(
    client: &mut BufReader<UnixStream>,
    mut db: Box<dyn Backend + Send>,
) -> Result<(), ClientError> {
    let mut tags = Vec::new();
    loop {
        let next_tag = client.read_line_alloc().await?;
        if next_tag.is_empty() {
            break;
        }
        tags.push(next_tag);
    }
    if tags.is_empty() {
        return Err(ClientError);
    }
    let words = db.get_words(&tags).await?;
    for word in words {
        client.write_all(word.as_bytes()).await?;
        client.write_all(b"\n").await?;
    }
    Ok(())
}
