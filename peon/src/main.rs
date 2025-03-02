use std::{
    error::Error,
    fs::File,
    io::{self, prelude::*, Read, Result as IoResult},
    net::TcpStream,
    os::{
        fd::OwnedFd,
        linux::net::SocketAddrExt,
        unix::net::{SocketAddr, UnixListener},
    },
    path::Path,
    process::{Child, ChildStderr, ChildStdout, Command, Stdio},
    sync::{LazyLock, Mutex},
};

use rustls::{
    pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer, ServerName},
    ClientConnection, RootCertStore, StreamOwned,
};
use serde::{Deserialize, Serialize};

const RAND: LazyLock<Mutex<File>> =
    LazyLock::new(|| Mutex::new(File::open("/dev/urandom").unwrap()));

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
fn main() {
    let proto = Protocol::Log {
        level: Level::Error,
        id: "Test".to_owned(),
        log: "Hello world".into(),
    };
    let proto_str = serde_json::to_string(&proto).unwrap();
    println!("{proto_str}");
}

struct ServerConn {
    stream: StreamOwned<ClientConnection, TcpStream>,
}

impl ServerConn {
    pub fn new(
        server: &str,
        ca_cert: &Path,
        client_cert: &Path,
        client_key: &Path,
    ) -> IoResult<Self> {
        let mut ca_store = RootCertStore::empty();
        ca_store
            .add(CertificateDer::from_pem_file(ca_cert).to_io()?)
            .to_io()?;

        let client_cert = vec![CertificateDer::from_pem_file(client_cert).to_io()?];
        let client_key = PrivateKeyDer::from_pem_file(client_key).to_io()?;
        let config = rustls::ClientConfig::builder()
            .with_root_certificates(ca_store)
            .with_client_auth_cert(client_cert, client_key)
            .to_io()?;
        let server_name = ServerName::try_from(server.to_owned()).to_io()?;
        let conn = ClientConnection::new(config.into(), server_name).to_io()?;
        let sock = TcpStream::connect(server)?;
        let stream = StreamOwned::new(conn, sock);
        Ok(ServerConn { stream })
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
        RAND.lock().unwrap().read_exact(&mut name[..])?;
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
