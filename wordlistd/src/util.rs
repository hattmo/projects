use std::io;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
pub trait GetLine {
    async fn read_line_alloc(&mut self) -> io::Result<String>;
}

impl<T> GetLine for BufReader<T>
where
    T: AsyncReadExt + Unpin,
{
    async fn read_line_alloc(&mut self) -> io::Result<String> {
        let mut buf = String::new();
        let _ = self.read_line(&mut buf).await?;
        buf = buf.trim().to_owned();
        Ok(buf)
    }
}
