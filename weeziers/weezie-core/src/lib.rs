#![feature(never_type)]
use std::sync::mpsc::{channel, Receiver, Sender};
pub struct Message {}

pub trait Capability {
    fn init(&mut self, channel: (Sender<Message>, Receiver<Message>));
}

pub struct CoreBuilder {
    id: u64,
    capabilities: Vec<Box<dyn Capability>>,
}

impl CoreBuilder {
    pub fn new(id: Option<u64>) -> Self {
        let (sender, receiver) = channel::<Message>();
        CoreBuilder {
            id: id.unwrap_or(0),
            capabilities: Vec::new(),
        }
    }
    pub fn register_capability(&mut self, tunnel: Box<dyn Capability>) {
        self.capabilities.push(tunnel)
    }
    pub fn run(&self) -> Result<!, &str> {
        let jobs = Vec::new();
        let (send, recv) = channel::<Message>();
        for cap in self.capabilities {
            std::thread::spawn(|| {});
        }
        Err("Exited")
    }
}
