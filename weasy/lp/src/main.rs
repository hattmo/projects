#![feature(tcplistener_into_incoming)]

use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
};

use axum::{response::IntoResponse, routing::get};
use proto::{request::Request, response::Response};
use rustls::{
    pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer},
    server::WebPkiClientVerifier,
    RootCertStore, ServerConfig, ServerConnection, Stream,
};
use x509_parser::prelude::*;

struct SessionQueues {
    responses: Vec<Response>,
    unhandled_requests: Vec<Request>,
    handled_requests: Vec<Request>,
}

type SessionContext = Arc<Mutex<SessionQueues>>;

fn main() {
    let ca_cert: &[u8] = fs::read("./crypto/ca.crt").unwrap().leak();
    let server_cert: &[u8] = fs::read("./crypto/server.crt").unwrap().leak();
    let server_key: &[u8] = fs::read("./crypto/server.key").unwrap().leak();
    let sessions = Mutex::new(HashMap::new());
    std::thread::scope(|t| {
        for conn in std::net::TcpListener::bind("0.0.0.0:1337")
            .unwrap()
            .into_incoming()
        {
            let Ok(conn) = conn else {
                println!("Failed to properly establish socket");
                continue;
            };
            let sessions = &sessions;
            t.spawn(move || {
                if let Err(e) = handle_conn(ca_cert, server_cert, server_key, conn, sessions) {
                    println!("{e}");
                };
            });
        }
    });
}

fn handle_conn(
    ca_cert: &[u8],
    server_cert: &[u8],
    server_key: &[u8],
    mut conn: std::net::TcpStream,
    sessions: &Mutex<HashMap<String, SessionContext>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = setup_ctx(ca_cert, server_cert, server_key).unwrap();
    while ctx.is_handshaking() {
        ctx.complete_io(&mut conn).unwrap();
    }
    let peer = get_peer(&ctx).ok_or("No Peer")?;
    let mut stream = Stream::new(&mut ctx, &mut conn);

    let mut session_lock = sessions.lock().unwrap();
    let session = session_lock
        .entry(peer)
        .or_insert(Arc::new(Mutex::new(SessionQueues {
            responses: Vec::new(),
            unhandled_requests: Vec::new(),
            handled_requests: Vec::new(),
        })))
        .clone();
    drop(session_lock);

    let mut session = session.lock().unwrap();

    let resp: Vec<Response> =
        bincode::decode_from_std_read(&mut stream, bincode::config::standard())?;
    session.responses.extend(resp);
    let req = std::mem::take(&mut session.unhandled_requests);
    bincode::encode_into_std_write::<&Vec<Request>, _, _>(
        &req,
        &mut stream,
        bincode::config::standard(),
    )?;
    session.handled_requests.extend(req);
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

#[derive(Debug)]
enum ConnectError {
    NoCerts,
    BadPem,
    BadConfig,
}

fn setup_ctx(
    ca_cert: &[u8],
    server_cert: &[u8],
    server_key: &[u8],
) -> Result<ServerConnection, ConnectError> {
    let mut store = RootCertStore::empty();
    store
        .add(CertificateDer::from_pem_slice(ca_cert).or(Err(ConnectError::BadPem))?)
        .or(Err(ConnectError::NoCerts))?;
    let verifier = WebPkiClientVerifier::builder(Arc::new(store))
        .build()
        .unwrap();
    let config = ServerConfig::builder()
        .with_client_cert_verifier(verifier)
        .with_single_cert(
            vec![CertificateDer::from_pem_slice(server_cert).or(Err(ConnectError::BadPem))?],
            PrivateKeyDer::from_pem_slice(server_key).or(Err(ConnectError::BadPem))?,
        )
        .unwrap();
    let ctx = ServerConnection::new(config.into()).or(Err(ConnectError::BadConfig))?;
    Ok(ctx)
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
