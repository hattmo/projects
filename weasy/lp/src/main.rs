#![feature(tcplistener_into_incoming)]

use std::{fs, sync::Arc};

use proto::{request::Request, response::Response};
use rustls::{
    pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer},
    server::WebPkiClientVerifier,
    RootCertStore, ServerConfig, ServerConnection,
};
use x509_parser::prelude::*;

fn main() {
    let ca_cert = fs::read("./crypto/ca.crt").unwrap().leak();
    let server_cert = fs::read("./crypto/server.crt").unwrap().leak();
    let server_key = fs::read("./crypto/server.key").unwrap().leak();

    for conn in std::net::TcpListener::bind("0.0.0.0:1337")
        .unwrap()
        .into_incoming()
    {
        let Ok(mut conn) = conn else {
            continue;
        };
        if let ControlFlow::Break(_) = fun_name(ca_cert, server_cert, server_key, conn) {
            return;
        }
    }
}

fn fun_name(
    ca_cert: &mut [u8],
    server_cert: &mut [u8],
    server_key: &mut [u8],
    mut conn: std::net::TcpStream,
) {
    let mut ctx = setup_ctx(ca_cert, server_cert, server_key).unwrap();
    while ctx.is_handshaking() {
        ctx.complete_io(&mut conn).unwrap();
    }
    let peer = get_peer(&ctx);
    let mut stream = rustls::Stream::new(&mut ctx, &mut conn);
    let _responses: Vec<Response> =
        match bincode::decode_from_std_read(&mut stream, bincode::config::standard()) {
            Ok(r) => r,
            Err(e) => {
                println!("{e}");
                return ControlFlow::Break(());
            }
        };
    bincode::encode_into_std_write::<Vec<Request>, _, _>(
        Vec::new(),
        &mut stream,
        bincode::config::standard(),
    )
    .unwrap();
    ControlFlow::Continue(())
}

fn get_peer(ctx: &ServerConnection) -> Option<String> {
    let (_, cert) = X509Certificate::from_der(ctx.peer_certificates()?.first()?).ok()?;
    let cn = cert
        .subject()
        .iter_common_name()
        .next()
        .map(AttributeTypeAndValue::as_str)?
        .ok()?;
    Some(cn.into())
}
#[derive(Debug)]
enum ConnectError {
    NoCerts,
    InvalidServerName,
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
