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

use std::{borrow::Cow, collections::HashMap, fmt::Display, io::Result as IoResult};

use clap::Parser;
use serde::{Deserialize, Serialize};

type FileList = HashMap<String, [u8; 32]>;
type FileEntry = (String, [u8; 32]);

#[derive(Serialize, Deserialize)]
enum Proto {
    FileList(FileList),
    Available,
    Transfer(FileEntry),
}

impl Display for Proto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: Cow<str> = match self {
            Proto::FileList(list) => {
                let list: Vec<&str> = list.into_iter().map(|(name, _)| name.as_str()).collect();
                let list = list.join(",");
                format!("File List: [{list}]").into()
            }
            Proto::Available => "Available".into(),
            Proto::Transfer((name, _)) => format!("Transfer: {}", name).into(),
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
