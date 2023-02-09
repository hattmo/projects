use serde::{Deserialize, Serialize};

use crate::{CapID, NodeID};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub src_node: NodeID,
    pub next_node: NodeID,
    pub dst_node: NodeID,
    pub src_cap: CapID,
    pub dst_cap: CapID,
    pub payload: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
enum CoreMessage {
    RouteAdvertisement(RouteAdvertisement),
}

#[derive(Serialize, Deserialize)]
struct RouteAdvertisement {}
