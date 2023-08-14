use std::os::fd::{FromRawFd, OwnedFd, RawFd};

use anyhow::Result;
use nix::unistd::pipe;
use tokio::{io::{unix::AsyncFd, AsyncRead}, fs::File};

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}

struct Inner {
    inner: File
}

impl AsyncRead for Inner {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let file = Box::pin(self.inner);
        file.poll_read
    }
}