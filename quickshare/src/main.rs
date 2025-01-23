#![feature(
    core_io_borrowed_buf,
    never_type,
    unwrap_infallible,
    maybe_uninit_as_bytes,
    read_buf
)]

mod client;
mod server;

use client::client_start;
use server::server_start;

use std::{borrow::Cow, fmt::Display, io::Result as IoResult, sync::Arc};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Proto {
    FileList(Arc<[FileEntry]>),
    FileChunk(FileChunk),
    Available,
    Transfer(FileEntry),
}

#[derive(Serialize, Deserialize)]
struct FileChunk {
    name: String,
    chunk: u64,
    content: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct FileEntry {
    hash: Vec<u8>,
    size: u128,
    name: String,
}

impl Display for Proto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: Cow<str> = match self {
            Proto::FileList(list) => {
                let list: Vec<String> = list.into_iter().map(|i| i.name.clone()).collect();
                let list = list.join(",");
                format!("File List: [{list}]").into()
            }
            Proto::FileChunk(file_chunk) => {
                format!("File Chunk: {} ({})", file_chunk.name, file_chunk.chunk).into()
            }
            Proto::Available => "Available".into(),
            Proto::Transfer(file_entry) => format!("Transfer: {}", file_entry.name).into(),
        };
        write!(f, "{text}")
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    serve: bool,
}

fn main() -> IoResult<()> {
    if Args::parse().serve {
        server_start()?;
    } else {
        client_start()?;
    };
    Ok(())
}
