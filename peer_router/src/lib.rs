mod job;
mod link;
mod router;
mod stream;
mod tls;
use std::{
    collections::HashMap,
    convert::Infallible,
    sync::mpsc::{
        channel,
        TryRecvError::{Disconnected, Empty},
    },
    time::Duration,
};

pub use job::{JobContext, JobError};
pub use stream::Stream;

use job::{Job, JobMessage};
use link::{Link, LinkContext, LinkMessage};
use router::Router;
use rustls::Connection;
use tls::TlsFactory;

pub struct Engine {
    node_id: u64,
    stale_time: Duration,
}

pub struct EngineConfig {
    pub node_id: u64,
    pub stale_time: Duration,
}

pub enum EngineError {
    FailedToLoadCerts,
}

struct Addr {
    pub node: u64,
    pub job: u64,
}

struct Message {
    pub from: Addr,
    pub to: Addr,
    pub stream: u16,
    pub seq: u32,
    pub ack: u32,
    pub payload: Vec<u8>,
}

enum EngineMessage {
    Message(Message),
}
impl Engine {
    pub fn new(config: EngineConfig) -> Engine {
        let EngineConfig {
            node_id,
            stale_time,
        } = config;
        Engine {
            node_id,
            stale_time,
        }
    }

    pub fn start(mut self) -> Result<Infallible, EngineError> {
        let mut jobs: HashMap<u64, Job> = HashMap::new();
        let mut links: HashMap<u64, Link> = HashMap::new();
        let mut streams: HashMap<u64, Stream> = HashMap::new();
        let mut job_count = 0;
        let mut link_count = 0;
        let mut stream_count = 0;
        let mut router = Router::new(self.stale_time);

        let tls_factory = TlsFactory::new(
            include_bytes!("../certs/ca.cert").to_vec(),
            include_bytes!("../certs/server.cert").to_vec(),
            include_bytes!("../certs/server.key").to_vec(),
        )
        .or(Err(EngineError::FailedToLoadCerts))?;

        loop {
            //
            // Handle messages from jobs
            //
            let messages: Vec<(u64, JobMessage)> = jobs
                .iter()
                .filter_map(|(job_id, job)| match job.receiver.try_recv() {
                    Ok(val) => Some((*job_id, val)),
                    Err(Empty) => None,
                    Err(Disconnected) => Some((*job_id, JobMessage::Kill)),
                })
                .collect();
            for (job_id, message) in messages {
                match message {
                    JobMessage::Kill => {
                        jobs.remove(&job_id);
                    }
                    JobMessage::NewJob(res) => {
                        res.send(create_job(self.node_id, &mut jobs, &mut job_count))
                            .unwrap();
                    }
                    JobMessage::ServerLink(res) => {
                        let Ok(conn) = tls_factory.create_server() else {
                            jobs.remove(&job_id);
                            continue;
                        };
                        if let Err(e) = res.send(create_link(conn, &mut links, &mut link_count)) {
                            jobs.remove(&job_id);
                        };
                    }
                    JobMessage::ClientLink(res) => {
                        let conn = tls_factory.create_client("Foobar").unwrap();
                        res.send(create_link(conn, &mut links, &mut link_count))
                            .unwrap();
                    }
                    JobMessage::Connect(node, job, res) => todo!(),
                    JobMessage::Accept(res) => {
                        todo!();
                    }
                };
            }
            //
            // Handle messages from links
            //
            let messages: Vec<_> = links
                .iter()
                .filter_map(|(link_id, link)| match link.receiver.try_recv() {
                    Ok(msg) => Some((*link_id, msg)),
                    Err(Empty) => None,
                    Err(Disconnected) => Some((*link_id, LinkMessage::Kill)),
                })
                .collect();
            for (link_id, message) in messages {
                match message {
                    LinkMessage::Kill => {
                        links.remove(&link_id);
                    }
                    LinkMessage::Message(msg) => {
                        self.handle_message(msg);
                    }
                }
            }
            router.purge_routes();
        }
    }

    fn handle_message(&mut self, mess: Message) {}
}

pub fn create_job(node_id: u64, jobs: &mut HashMap<u64, Job>, job_count: &mut u64) -> JobContext {
    let (to_job, from_engine) = channel();
    let (to_engine, from_job) = channel();
    let job = *job_count;
    *job_count += 1;
    jobs.insert(
        job,
        Job {
            sender: to_job,
            receiver: from_job,
        },
    );
    JobContext::new(node_id, job, to_engine, from_engine)
}

fn create_link(
    conn: Connection,
    links: &mut HashMap<u64, Link>,
    link_count: &mut u64,
) -> LinkContext {
    let (to_link, from_engine) = channel();
    let (to_engine, from_link) = channel();
    let link_id = *link_count;
    *link_count += 1;
    links.insert(
        link_id,
        Link {
            sender: to_link,
            receiver: from_link,
        },
    );
    LinkContext::new(to_engine, from_engine, conn)
}
