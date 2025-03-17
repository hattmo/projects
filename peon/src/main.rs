mod proto {
    tonic::include_proto!("service");
}

use std::{
    error::Error,
    io::{self, Result as IoResult},
    os::{
        fd::OwnedFd,
        linux::net::SocketAddrExt,
        unix::net::{SocketAddr, UnixListener, UnixStream as StdUnixStream},
    },
    path::Path,
    process::{Child, ChildStderr, ChildStdout, Command, Stdio},
};

use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::net::UnixStream as TokioUnixStream;

use proto::great_hall_client::GreatHallClient;

#[derive(Serialize, Deserialize)]
enum Protocol {
    Log {
        level: Level,
        id: String,
        log: Vec<u8>,
    },
    Request {
        id: String,
        data: Vec<u8>,
    },
}
#[derive(Serialize, Deserialize)]
enum Level {
    Error,
    Info,
}

#[tokio::main]
async fn main() {
    let worker = Worker::new(Path::new("/foo/bar"), [].as_slice());
    let mut client = GreatHallClient::connect("foo.com").await.unwrap();
    let mut commands = client.get_commands(()).await.unwrap().into_inner();
    while let Ok(Some(command)) = commands.message().await {
        command.
    }
}

struct Worker {
    addr: SocketAddr,
    log_stream: ChildStdout,
    err_log_stream: ChildStderr,
    child: Child,
}

impl Worker {
    pub fn new(path: &Path, args: &[&str]) -> IoResult<Self> {
        let mut name = [0u8; 20];
        rand::thread_rng().fill(&mut name);
        let addr = SocketAddr::from_abstract_name(&name)?;
        let sock: OwnedFd = UnixListener::bind_addr(&addr)?.into();
        let mut child = Command::new(path)
            .args(args)
            .stdin(sock)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let err_log_stream = child.stderr.take().to_io()?;
        let log_stream = child.stdout.take().to_io()?;
        Ok(Worker {
            addr,
            log_stream,
            err_log_stream,
            child,
        })
    }
    pub fn connect(&self) -> IoResult<TokioUnixStream> {
        let conn = StdUnixStream::connect_addr(&self.addr)?;
        conn.set_nonblocking(true)?;
        let conn = TokioUnixStream::from_std(conn)?;
        Ok(conn)
    }
}

trait ToIoResult<T> {
    fn to_io(self) -> IoResult<T>;
}

impl<T, E> ToIoResult<T> for Result<T, E>
where
    T: Sync + Send + 'static,
    E: Error + Sync + Send + 'static,
{
    fn to_io(self) -> IoResult<T> {
        self.map_err(|e| io::Error::other(e))
    }
}

impl<T> ToIoResult<T> for Option<T>
where
    T: Sync + Send + 'static,
{
    fn to_io(self) -> IoResult<T> {
        self.ok_or(io::Error::other("Option is None"))
    }
}
