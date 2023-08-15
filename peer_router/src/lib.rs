use std::collections::{HashMap, VecDeque};
use tunnel::Tunnel;
mod tunnel;

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
        todo!()
    }
}
