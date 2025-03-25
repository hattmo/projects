mod proto {
    tonic::include_proto!("service");
}

use std::{
    collections::HashMap,
    error::Error,
    ffi::{OsStr, OsString},
    io::{self, Result as IoResult},
    os::{
        fd::OwnedFd,
        linux::net::SocketAddrExt,
        unix::{
            net::{SocketAddr, UnixListener, UnixStream as StdUnixStream},
            process::CommandExt,
        },
    },
    path::{Path, PathBuf},
    process::{Child, ChildStderr, ChildStdout, Command, Stdio},
};

use libc::MNT_DETACH;
use rand::Rng;
use tokio::net::UnixStream as TokioUnixStream;

use proto::great_hall_client::GreatHallClient;

#[tokio::main]
async fn main() {
    //let worker = Worker::new(Path::new("/foo/bar"), [].as_slice());
    let mut client = GreatHallClient::connect("foo.com").await.unwrap();
    let mut commands = client.get_commands(()).await.unwrap().into_inner();
    while let Ok(Some(proto::Command {
        command,
        id,
        module,
        args,
        env,
        preserve_env,
    })) = commands.message().await
    {}
}

struct Worker {
    addr: SocketAddr,
    log_stream: ChildStdout,
    err_log_stream: ChildStderr,
    child: Child,
}

impl Worker {
    pub fn new(
        path: impl AsRef<Path>,
        command: impl AsRef<OsStr>,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
        envs: impl IntoIterator<Item = (impl AsRef<OsStr>, impl AsRef<OsStr>)>,
        clean_env: bool,
    ) -> IoResult<Self> {
        let mut name = [0u8; 20];
        rand::thread_rng().fill(&mut name);
        let addr = SocketAddr::from_abstract_name(&name)?;
        let sock: OwnedFd = UnixListener::bind_addr(&addr)?.into();
        let mut child = Command::new(command);
        if clean_env {
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
        })
    }
    pub fn connect(&self) -> IoResult<TokioUnixStream> {
        let conn = StdUnixStream::connect_addr(&self.addr)?;
        conn.set_nonblocking(true)?;
        let conn = TokioUnixStream::from_std(conn)?;
        Ok(conn)
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
