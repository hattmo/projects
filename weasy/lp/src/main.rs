use std::{
    collections::HashMap,
    fs,
    future::pending,
    path::PathBuf,
    sync::{Arc, Mutex, MutexGuard},
};

use bincode::config::{self, Configuration};
use proto::{
    request::{Open, Request, RequestType},
    response::Response,
};
use rustls::{
    pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer},
    server::WebPkiClientVerifier,
    RootCertStore, ServerConfig, ServerConnection, Stream,
};
use x509_parser::prelude::*;
use zbus::{connection, interface};

struct Weasy {
    store: AgentStore,
}

#[interface(name = "com.hattmo.Weasy1")]
impl Weasy {
    fn request_open(&self, agent: &str, sess: u64, path: PathBuf) {
        let agent_ctx = self.store.get_agent_context(agent);
        let seq = agent_ctx.next_seq(sess);
        let req = Request {
            sess,
            seq,
            req: RequestType::Open(Open { path }),
        };
        agent_ctx.put_request(req);
    }
}

#[derive(Default, Clone)]
struct AgentStore(Arc<Mutex<HashMap<String, AgentContext>>>);

impl AgentStore {
    fn new() -> Self {
        AgentStore::default()
    }

    fn get_agent_context(&self, agent: &str) -> AgentContext {
        let mut lock = self.0.lock().unwrap();
        let context = lock.entry(agent.to_owned()).or_default();
        context.clone()
    }
}

struct Transaction {
    req: Request,
    res: Vec<Response>,
}

impl From<Request> for Transaction {
    fn from(value: Request) -> Self {
        Transaction {
            req: value,
            res: Vec::new(),
        }
    }
}

#[derive(Default, Clone)]
struct AgentContext {
    sessions: Arc<Mutex<HashMap<u64, u64>>>,
    transactions: Arc<Mutex<Vec<Transaction>>>,
}

struct PendingRequests<'a> {
    lock: MutexGuard<'a, Vec<Transaction>>,
}

impl<'a> PendingRequests<'a> {
    fn iter(&'a self) -> impl Iterator<Item = &'a Request> {
        let ret = self
            .lock
            .iter()
            .filter(|i| i.res.is_empty())
            .map(|i| &i.req);
        ret
    }
}

impl AgentContext {
    fn put_request(&self, req: Request) {
        let mut lock = self.transactions.lock().unwrap();
        lock.push(req.into());
    }

    fn put_responses(&self, ress: impl IntoIterator<Item = Response>) {
        let mut lock = self.transactions.lock().unwrap();
        for new_res in ress {
            if let Some(i) = lock
                .iter_mut()
                .find(|i| i.req.sess == new_res.sess && i.req.seq == new_res.seq)
            {
                i.res.push(new_res)
            };
        }
    }

    fn pending_requests(&self) -> PendingRequests<'_> {
        PendingRequests {
            lock: self.transactions.lock().unwrap(),
        }
    }

    fn next_seq(&self, sess: u64) -> u64 {
        let mut lock = self.sessions.lock().unwrap();
        let seq = lock.entry(sess).or_default();
        *seq += 1;
        *seq
    }
}

fn main() {
    let ca_cert = fs::read("./crypto/ca.crt").unwrap();
    let server_cert = fs::read("./crypto/server.crt").unwrap();
    let server_key = fs::read("./crypto/server.key").unwrap();
    let config = new_server_config(&ca_cert, &server_cert, &server_key).unwrap();
    let store = AgentStore::new();
    std::thread::scope(|t| {
        let dbus_store = store.clone();
        t.spawn(|| {
            let ret: Result<(), Box<str>> =
                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    let _conn = connection::Builder::session()
                        .err_str()?
                        .name("com.hattmo.Weasy")
                        .err_str()?
                        .serve_at("/com/hattmo/Weasy", Weasy { store: dbus_store })
                        .err_str()?
                        .build()
                        .await
                        .err_str()?;
                    pending::<()>().await;
                    Ok(())
                });
            if let Err(e) = ret {
                println!("Error: {e}");
            }
        });
        for conn in std::net::TcpListener::bind("0.0.0.0:1337")
            .unwrap()
            .incoming()
        {
            let Ok(conn) = conn else {
                println!("Failed to properly establish socket");
                continue;
            };
            let store = store.clone();
            let config = config.clone();
            t.spawn(move || {
                if let Err(e) = handle_conn(config, conn, store) {
                    println!("{e}");
                };
            });
        }
    });
}

const BC_CONF: Configuration = config::standard();

fn handle_conn(
    config: Arc<ServerConfig>,
    mut conn: std::net::TcpStream,
    store: AgentStore,
) -> Result<(), Box<str>> {
    let mut ctx = ServerConnection::new(config).err_str()?;
    while ctx.is_handshaking() {
        ctx.complete_io(&mut conn).err_str()?;
    }
    let peer = get_peer(&ctx).ok_or("Unknow peer in cert")?;
    let agent_ctx = store.get_agent_context(&peer);
    let mut stream = Stream::new(&mut ctx, &mut conn);

    let res: Vec<Response> = bincode::decode_from_std_read(&mut stream, BC_CONF).err_str()?;
    agent_ctx.put_responses(res);

    let requests = agent_ctx.pending_requests();
    let requests: Box<[&Request]> = requests.iter().collect();
    bincode::encode_into_std_write(&requests, &mut stream, BC_CONF).err_str()?;

    Ok(())
}

fn get_peer(ctx: &ServerConnection) -> Option<String> {
    let (_, cert) = X509Certificate::from_der(ctx.peer_certificates()?.first()?).ok()?;
    let cn = cert
        .subject()
        .iter_common_name()
        .next()
        .map(AttributeTypeAndValue::as_str)?
        .ok()?
        .to_owned();
    Some(cn)
}

fn new_server_config(
    ca_cert: &[u8],
    server_cert: &[u8],
    server_key: &[u8],
) -> Result<Arc<ServerConfig>, Box<str>> {
    let mut store = RootCertStore::empty();
    store
        .add(CertificateDer::from_pem_slice(ca_cert).err_str()?)
        .err_str()?;
    let verifier = WebPkiClientVerifier::builder(Arc::new(store))
        .build()
        .unwrap();
    let config = ServerConfig::builder()
        .with_client_cert_verifier(verifier)
        .with_single_cert(
            vec![CertificateDer::from_pem_slice(server_cert).err_str()?],
            PrivateKeyDer::from_pem_slice(server_key).err_str()?,
        )
        .unwrap();
    Ok(config.into())
}

trait ErrorString<T> {
    fn err_str(self) -> Result<T, Box<str>>;
}

impl<T, E> ErrorString<T> for Result<T, E>
where
    E: ToString,
{
    fn err_str(self) -> Result<T, Box<str>> {
        self.map_err(|e| e.to_string().into())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let mut buf1 = Vec::new();
        let mut buf2 = Vec::new();
        let val = vec![1, 2, 3];
        bincode::encode_into_std_write(&val, &mut buf1, bincode::config::standard()).unwrap();
        bincode::encode_into_std_write(val, &mut buf2, bincode::config::standard()).unwrap();
        assert_eq!(buf1, buf2)
    }
}
