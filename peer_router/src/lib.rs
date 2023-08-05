use std::{
    collections::{HashMap, VecDeque},
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};
use x25519_dalek::{PublicKey, SharedSecret, StaticSecret};
struct Engine {
    node: u32,
    buffers: HashMap<(u32, u32), VecDeque<u8>>,
    tunnel_count: u16,
}

impl Engine {
    pub fn new(node: u32) -> Engine {
        Engine {
            node,
            buffers: HashMap::new(),
            tunnel_count: 0,
        }
    }

    pub fn create_tunnel(&mut self) -> Tunnel {
        self.tunnel_count += 1;
        Tunnel::new(self.tunnel_count, self)
    }

    pub fn connect(&self, node: u32, service: u32) -> Result<EngineConnection, EngineError> {
        todo!()
    }

    pub fn listen(&self, service: u32) -> Result<EngineListener, EngineError> {
        todo!()
    }
}

pub struct Tunnel<'a> {
    id: u16,
    engine: &'a Engine,
    secret_state: TunnelState,
}

impl<'a> Tunnel<'a> {
    fn new(id: u16, engine: &'a Engine) -> Tunnel {
        let private = StaticSecret::new(rand_core::OsRng);
        Tunnel {
            id,
            engine,
            secret_state: TunnelState::NeedCertificate,
        }
    }

    pub fn put_packet(&mut self, packet: Vec<u8>) {
        let Ok(message) = bincode::deserialize::<TunnelMessage>(&packet) else {
            return;
        };
        match (message, &mut self.secret_state) {
            (TunnelMessage::PublicKey(public), TunnelState::Waiting(ephemeral_secret)) => {
                let shared_secret = ephemeral_secret.diffie_hellman(&public);
                self.secret_state = TunnelState::Established(shared_secret);
            }
            (TunnelMessage::Packet(_), _) => todo!(),
            (_, _) => todo!(),
        }
        todo!()
    }

    pub fn get_packet(&self) -> Vec<u8> {
        todo!()
    }
}

enum TunnelState {
    NeedCertificate,
    Waiting(StaticSecret),
    Established(SharedSecret),
}

#[derive(Serialize, Deserialize)]
pub enum TunnelMessage {
    Certificate(Vec<u8>),
    PublicKey(PublicKey),
    Packet(Vec<u8>),
}






















struct EngineConnection<'a> {
    engine: &'a Engine,
}

impl Write for EngineConnection<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

impl Read for EngineConnection<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}

struct EngineListener;

impl EngineListener {
    pub fn accept(&self) -> Result<EngineConnection, EngineError> {
        todo!()
    }
}

struct EngineError;

#[cfg(test)]
mod test {

    #[test]
    fn test() {}
}
