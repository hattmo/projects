#![feature(iter_next_chunk)]

mod proto {
    tonic::include_proto!("service");
}

mod fast_cgi;

use std::{
    collections::HashMap,
    error::Error,
    ffi::OsStr,
    hash::{DefaultHasher, Hash, Hasher},
    io::{self, Result as IoResult},
    num::Wrapping,
    os::{
        fd::OwnedFd,
        linux::net::SocketAddrExt,
        unix::{
            net::{SocketAddr, UnixListener, UnixStream as StdUnixStream},
            process::CommandExt,
        },
    },
    path::Path,
    process::{Child, ChildStderr, ChildStdout, Command, Stdio},
};

use libc::MNT_DETACH;
use rand::Rng;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream as TokioUnixStream,
};

use fast_cgi::{Record, RecordBytes};
use proto::{great_hall_client::GreatHallClient, Output};

#[derive(PartialEq, Eq, Hash)]
struct Context<'a> {
    module: &'a str,
    command: &'a str,
    args: &'a [String],
    envs: &'a [(String, String)],
}

#[tokio::main]
async fn main() {
    let mut workers = HashMap::new();
    let mut client = GreatHallClient::connect("foo.com").await.unwrap();
    let mut commands = client.get_commands(()).await.unwrap().into_inner();
    while let Ok(Some(proto::Command {
        module,
        command,
        args,
        envs,
        preserve_env,

        id,
        params,
        data,
    })) = commands.message().await
    {
        let mut envs: Vec<(String, String)> = envs.into_iter().collect();
        envs.sort();
        let context = Context {
            module: &module,
            command: &command,
            args: &args,
            envs: &envs,
        };
        let mut hasher = DefaultHasher::new();
        context.hash(&mut hasher);
        let hash = hasher.finish();
        let worker = workers
            .entry(hash)
            .or_insert_with(|| Worker::new(module, command, args, envs, preserve_env).unwrap());
        let mut connection = worker.connect().await.unwrap();
        let mut client = client.clone();
        tokio::task::spawn(async move {
            connection.send_stdin(&data).await.unwrap();
            let output = Output {
                parent: id,
                output: vec![],
            };
            client.send_output(output).await;
        });
    }
}

struct Worker {
    addr: SocketAddr,
    log_stream: ChildStdout,
    err_log_stream: ChildStderr,
    child: Child,
    request_id: Wrapping<u16>,
}

impl Worker {
    pub fn new(
        path: impl AsRef<Path>,
        command: impl AsRef<OsStr>,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
        envs: impl IntoIterator<Item = (impl AsRef<OsStr>, impl AsRef<OsStr>)>,
        preserve_env: bool,
    ) -> IoResult<Self> {
        let mut name = [0u8; 20];
        rand::thread_rng().fill(&mut name);
        let addr = SocketAddr::from_abstract_name(&name)?;
        let sock: OwnedFd = UnixListener::bind_addr(&addr)?.into();
        let mut child = Command::new(command);
        if !preserve_env {
            child.env_clear();
        }
        child
            .args(args)
            .envs(envs)
            .stdin(sock)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(path)
            .process_group(0);
        let mut child = unsafe { child.pre_exec(move || setup_jail()).spawn()? };
        let err_log_stream = Option::take(&mut child.stderr).to_io()?;
        let log_stream = Option::take(&mut child.stdout).to_io()?;
        Ok(Worker {
            addr,
            log_stream,
            err_log_stream,
            child,
            request_id: Wrapping(1u16),
        })
    }

    pub async fn connect(&mut self) -> IoResult<WorkerSession> {
        let conn = StdUnixStream::connect_addr(&self.addr)?;
        conn.set_nonblocking(true)?;
        let mut conn = TokioUnixStream::from_std(conn)?;
        let start_record = fast_cgi::Record {
            request_id: self.request_id.0,
            content: fast_cgi::RecordType::BeginRequest {
                role: fast_cgi::Role::Responder,
                keep_conn: false,
            },
        };
        let RecordBytes {
            header,
            content,
            padding,
        } = start_record.write_record().unwrap();
        conn.write_all(&header).await?;
        conn.write_all(&content).await?;
        conn.write_all(&padding).await?;
        let session = WorkerSession {
            stream: conn,
            request_id: self.request_id,
        };
        self.request_id += 1;
        Ok(session)
    }
}

struct WorkerSession {
    stream: TokioUnixStream,
    request_id: Wrapping<u16>,
}

impl WorkerSession {
    async fn send_stdin(&mut self, data: &[u8]) -> IoResult<()> {
        let record = fast_cgi::Record {
            request_id: self.request_id.0,
            content: fast_cgi::RecordType::Stdin { data },
        };
        let RecordBytes {
            header,
            content,
            padding,
        } = record.write_record().unwrap();
        self.stream.write_all(&header).await?;
        self.stream.write_all(&content).await?;
        self.stream.write_all(&padding).await?;
        Ok(())
    }
    async fn get_data<'a>(&mut self, buffer: &'a mut [u8]) -> IoResult<Record<'a>> {
        self.stream.read(buffer).await?;
        let (record, _) = fast_cgi::Record::parse_record(buffer).to_io()?;
        Ok(record)
    }
}

fn setup_jail() -> Result<(), io::Error> {
    unsafe {
        libc::unshare(libc::CLONE_NEWNS);
        libc::syscall(libc::SYS_pivot_root, c".".as_ptr(), c".".as_ptr());
        libc::umount2(c".".as_ptr(), MNT_DETACH);
    }
    Ok(())
}

trait ToIoResult<T> {
    fn to_io(self) -> IoResult<T>;
}

impl<T, E> ToIoResult<T> for Result<T, E>
where
    T: Sync + Send,
    E: Error + Sync + Send + 'static,
{
    fn to_io(self) -> IoResult<T> {
        self.map_err(|e| io::Error::other(e))
    }
}

impl<T> ToIoResult<T> for Option<T>
where
    T: Sync + Send,
{
    fn to_io(self) -> IoResult<T> {
        self.ok_or(io::Error::other("Option is None"))
    }
}
