use std::io;

use clap::{Parser, Subcommand};
use tokio::{
    io::{AsyncWrite, AsyncWriteExt, BufWriter},
    net::UnixStream,
};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
#[command()]
enum Command {
    Add { word: String, tags: Vec<String> },
    Get { tags: Vec<String> },
}

pub async fn main() -> Result<(), CliError> {
    println!("client mode");
    let conn = UnixStream::connect("./sock").await?;
    match Args::parse().command {
        Command::Add { word, tags } => handle_add(&word, &tags, &mut BufWriter::new(conn)).await?,
        Command::Get { tags } => todo!(),
    }
    Ok(())
}

async fn handle_add<T, S>(word: &str, tags: &[S], conn: &mut BufWriter<T>) -> Result<(), CliError>
where
    T: AsyncWrite + Unpin,
    S: AsRef<str>,
{
    conn.write_all(b"add\n").await?;
    conn.write_all(word.as_bytes()).await?;
    conn.write_all(b"\n").await?;
    for tag in tags {
        conn.write_all(tag.as_ref().as_bytes()).await?;
        conn.write_all(b"\n").await?;
    }
    conn.write_all(b"\n").await?;
    Ok(())
}

pub enum CliError {
    Io(io::Error),
}
impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
