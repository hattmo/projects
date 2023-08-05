use anyhow::Result;
use rand::Fill;
use std::{
    os::{
        fd::OwnedFd,
        linux::net::SocketAddrExt,
        unix::net::{SocketAddr, UnixListener},
    },
    process::{id, Stdio},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::UnixStream,
    process::Command,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", id());
    let mut add_name = [0; 32];
    add_name.try_fill(&mut rand::thread_rng())?;
    let addr = SocketAddr::from_abstract_name(add_name)?;
    let listener = UnixListener::bind_addr(&addr)?;
    let fd: OwnedFd = listener.into();
    let stdio = Stdio::from(fd);
    let mut child = Command::new("python")
        .arg("-m")
        .arg("child")
        .stdin(stdio)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let stream = std::os::unix::net::UnixStream::connect_addr(&addr)?;
    stream.set_nonblocking(true)?;
    let mut stream = UnixStream::from_std(stream)?;
    stream.write_all(b"hello world\n").await?;
    let stdout = child.stdout.take().unwrap();
    let stdout = BufReader::new(stdout);
    let mut lines = stdout.lines();
    while let Some(line) = lines.next_line().await? {
        println!("child: {}", line);
    }
    Ok(())
}
