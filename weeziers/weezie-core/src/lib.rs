#![feature(never_type)]
#![feature(io_error_other)]
use messages::Message;
use rand::random;
use std::collections::HashMap;
pub mod extensions;
mod messages;
type NodeID = u32;
type CapID = u16;
pub const CORE_CAP: u16 = 0;

pub trait Extension {
    fn init(&self) -> CapID;
    fn exec(&mut self, message: &mut Vec<Message>) -> Vec<Message>;
}

pub struct Core {
    id: NodeID,
    cache: HashMap<CapID, Vec<Message>>,
    unprocessed: Vec<Message>,
    extentions: Vec<(CapID, Box<dyn Extension>)>,
    routes: HashMap<NodeID, (CapID, NodeID)>,
}

impl Core {
    pub fn new() -> Self {
        Self {
            id: random(),
            cache: HashMap::new(),
            unprocessed: Vec::new(),
            extentions: Vec::new(),
            routes: HashMap::new(),
        }
    }
    pub fn register_extension(&mut self, extension: Box<dyn Extension>) {
        let caps = extension.init();
        self.extentions.push((caps, extension));
    }
    pub fn run(&mut self) -> Result<!, ()> {
        loop {
            for (cap_id, ext) in self.extentions.iter_mut() {
                let inc = self.cache.entry(*cap_id).or_default();
                let mut out = ext.exec(inc);
                self.unprocessed.append(&mut out);
            }

            for mut mess in self.unprocessed.drain(0..self.unprocessed.len()) {
                // TODO: Check if signed message

                if mess.dst_node != self.id {
                    if let Some((thru, via)) = self.routes.get(&mess.dst_node) {
                        mess.next_node = *via;
                        self.cache.entry(*thru).or_default().push(mess);
                    }
                } else {
                    if mess.dst_cap == CORE_CAP {}
                }
            }
        }
    }
}
