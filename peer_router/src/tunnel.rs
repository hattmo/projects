use std::{
    collections::HashMap,
    io::{prelude::*, BufRead, BufReader},
};

use rustls::{
    ClientConfig, ClientConnection, Connection, PrivateKey, RootCertStore, ServerConfig,
    ServerConnection,
};
use serde::{Deserialize, Serialize};

struct Session {
    tls_state: Connection,
}

pub struct Tunnel<const N: usize> {
    id: u16,
    sessions: HashMap<u16, Session>,
}

impl<const N: usize> Tunnel<N> {
    fn new(id: u16) -> Tunnel<N> {
        Tunnel {
            id,
            sessions: HashMap::new(),
        }
    }

    // pub fn initiate(&mut self) {
    //     let config = ClientConfig::builder()
    //         .with_safe_defaults()
    //         .with_root_certificates(RootCertStore::empty())
    //         .with_no_client_auth();
    //     let conn = ClientConnection::new(config.into(), "localhost".try_into().unwrap()).unwrap();
    // }

    pub fn process_packet(&mut self, packet: impl AsRef<[u8]>) -> Option<[u8; N]> {
        let out = [0; N];
        let packet = packet.as_ref();
        let packet: TunnelPacket = bincode::deserialize(packet).unwrap();
        let session = self.sessions.entry(packet.id).or_insert_with(|| {
            let conf = ServerConfig::builder()
                .with_safe_defaults()
                .with_no_client_auth()
                .with_single_cert(Vec::new(), PrivateKey(Vec::new()))
                .unwrap();
            let tls_state = ServerConnection::new(conf.into()).unwrap();
            Session {
                tls_state: tls_state.into(),
            }
        });
        let tls = &mut session.tls_state;
        tls.read_tls(&mut packet.data.as_slice()).unwrap();
        tls.process_new_packets().unwrap();
        let plain_in = Vec::new();
        tls.reader().
        Some(out)
    }
}

#[derive(Serialize, Deserialize)]
struct TunnelPacket {
    id: u16,
    seq: u32,
    conf: u32,
    data: Vec<u8>,
}
