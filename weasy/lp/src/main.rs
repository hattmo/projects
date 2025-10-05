#![feature(tcplistener_into_incoming)]

use std::{fs, sync::Arc};

use proto::{request::Request, response::Response};
use rustls::{
    pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer},
    server::WebPkiClientVerifier,
    RootCertStore, ServerConfig, ServerConnection,
};
fn main() {
    let ca_cert = fs::read("./ca.pem").unwrap().leak();
    let server_cert = fs::read("./client.crt").unwrap().leak();
    let server_key = fs::read("./client.key").unwrap().leak();

    for conn in std::net::TcpListener::bind("0.0.0.0:1337")
        .unwrap()
        .into_incoming()
    {
        let Ok(mut conn) = conn else {
            continue;
        };
        let mut ctx = setup_ctx(ca_cert, server_cert, server_key).unwrap();
        let mut stream = rustls::Stream::new(&mut ctx, &mut conn);
        let responses: Vec<Response> =
            bincode::decode_from_std_read(&mut stream, bincode::config::standard()).unwrap();
        println!("got");
        bincode::encode_into_std_write::<Vec<Request>, _, _>(
            Vec::new(),
            &mut stream,
            bincode::config::standard(),
        )
        .unwrap();
    }
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
