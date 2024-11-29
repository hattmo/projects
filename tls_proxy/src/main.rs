use rcgen::{ExtendedKeyUsagePurpose, IsCa, KeyUsagePurpose};
use ring::{self, signature::Ed25519KeyPair};
use rustls::{
    crypto::ring::sign::any_supported_type,
    pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer},
    server::{ClientHello, ResolvesServerCert},
    sign::CertifiedKey,
    ServerConfig,
};
use std::{
    collections::HashMap,
    process::Output,
    sync::{Arc, Mutex},
};

fn main() {
    ServerConfig::builder()
        .with_no_client_auth()
        .with_cert_resolver(Arc::new(Resolver::default()));
    println!("Hello, world!");
}

#[derive(Debug)]
struct Resolver {
    certs: Mutex<HashMap<String, Arc<CertifiedKey>>>,
}

impl Default for Resolver {
    fn default() -> Self {
        Self {
            certs: Mutex::new(HashMap::new()),
        }
    }
}

impl ResolvesServerCert for Resolver {
    fn resolve(&self, client_hello: ClientHello<'_>) -> Option<Arc<CertifiedKey>> {
        client_hello
            .server_name()
            .and_then(|name| get_cert(name, self))
    }
}

fn get_cert(name: &str, resolver: &Resolver) -> Option<Arc<CertifiedKey>> {
    Some(
        resolver
            .certs
            .lock()
            .unwrap()
            .entry(name.to_string())
            .or_insert_with_key(|key| generate_cert(key))
            .clone(),
    )
}

fn generate_cert(key: &str) -> Arc<CertifiedKey> {
    let mut params = rcgen::CertificateParams::new([key.to_string()].as_ref()).unwrap();
    params.not_before = rcgen::date_time_ymd(2024, 1, 1);
    params.not_after = rcgen::date_time_ymd(2025, 1, 1);
    params.is_ca = IsCa::NoCa;
    params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ServerAuth];

    let Output { stdout: output, .. } = std::process::Command::new("openssl")
        .args(&["foo"])
        .output()
        .unwrap();
    let cert_der = Vec::leak(output);
    let key_der = [];
    CertifiedKey::new(
        vec![CertificateDer::from_slice(cert_der)],
        any_supported_type(&PrivateKeyDer::from_pem_slice(&key_der).unwrap()).unwrap(),
    )
    .into()
}
