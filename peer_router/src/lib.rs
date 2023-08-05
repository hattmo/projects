#![feature(generic_const_exprs)]

use std::{
    collections::{HashMap, VecDeque},
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};
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

    pub fn create_tunnel<const N: usize>(&mut self) -> Tunnel<N> {
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

pub struct Tunnel<'a, const N: usize> {
    id: u16,
    engine: &'a Engine,
    secret_state: SecretState,
}

impl<'a, const N: usize> Tunnel<'a, N> {
    fn new(id: u16, engine: &'a Engine) -> Tunnel<N> {
        let private = EphemeralSecret::new(rand_core::OsRng);
        let foo = PublicKey::from(&private);
        Tunnel {
            id,
            engine,
            secret_state: SecretState::Waiting,
        }
    }

    pub fn put_packet(&mut self, packet: Vec<u8>) {
        let Ok(message) = bincode::deserialize::<TunnelMessage>(&packet) else {
            return;
        };
        match (message, &mut self.secret_state) {
            (TunnelMessage::PublicKey(public), SecretState::Ready(foo)) => {
                let state = std::mem::replace(&mut self.secret_state, SecretState::Waiting);
            }
            (TunnelMessage::NeedPublicKey, _) => todo!(),
            (TunnelMessage::Packet(_), _) => todo!(),
            (_, _) => todo!(),
        }
        todo!()
    }

    pub fn get_packet(&self) -> [u8; N * 32] {
        todo!()
    }
}

enum SecretState {
    Waiting,
    Ready(EphemeralSecret),
    Established(SharedSecret),
}

#[derive(Serialize, Deserialize)]
pub enum TunnelMessage {
    PublicKey(PublicKey),
    NeedPublicKey,
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
