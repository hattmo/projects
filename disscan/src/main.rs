use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use disscan::{Consumer, Queue};
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
    sync::broadcast::Receiver,
};

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: u64,
    name: String,
    targets: Vec<String>,
    args: Vec<Vec<u8>>,
}

async fn client_read_job(mut reader: OwnedReadHalf) -> Result<()> {
    let mut buf = [0u8; 256];
    loop {
        let frame = reader.read_u16().await?.into();
        let buf = &mut buf[..frame];
        reader.read_exact(buf).await?;
        let val = serde_json::from_slice::<serde_json::Value>(buf)?;
    }
    Ok(())
}

async fn client_write_job(
    mut writer: OwnedWriteHalf,
    mut tasks: Consumer<Task>,
) -> Result<()> {
    loop {
        let task = tasks.pop().await.or(Err(anyhow!("error")))?;
        let buf = serde_json::to_vec(&task)?;
        writer.write_u32(buf.len().try_into()?).await?;
        writer.write_all(&buf).await?;
    }
}

async fn handle_client(mut stream: TcpStream) -> Result<()> {
    let (reader, writer) = stream.into_split();
    tokio::spawn(client_read_job(reader));
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let serve = TcpListener::bind("0.0.0.0:9999").await?;
    while let Ok((client, _)) = serve.accept().await {
        tokio::spawn(handle_client(client));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut val = Task {
            args: vec![],
            id: 0,
            name: "".to_string(),
            targets: vec![],
        };
        val.args.push(b"helloworld".to_vec());
        let buf = serde_json::to_string_pretty(&val).unwrap();
        println!("{}", buf);
    }
}
