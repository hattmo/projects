use crate::{stream::Stream, EngineMessage, LinkContext};
use std::sync::mpsc::{Receiver, Sender};

pub(crate) struct Job {
    pub sender: Sender<EngineMessage>,
    pub receiver: Receiver<JobMessage>,
}

pub struct JobContext {
    node: u64,
    job: u64,
    sender: Sender<JobMessage>,
    receiver: Receiver<EngineMessage>,
}

pub enum JobError {
    FailedToSendToEngine,
    FailedToReceiveFromEngine,
}

pub enum JobMessage {
    Kill,
    NewJob(oneshot::Sender<JobContext>),
    ServerLink(oneshot::Sender<LinkContext>),
    ClientLink(oneshot::Sender<LinkContext>),
    Connect(u64, u64, oneshot::Sender<Stream>),
    Accept(oneshot::Sender<(u64, u64, Stream)>),
}

impl JobContext {
    pub(crate) fn new(
        node: u64,
        job: u64,
        sender: Sender<JobMessage>,
        receiver: Receiver<EngineMessage>,
    ) -> JobContext {
        JobContext {
            node,
            job,
            sender,
            receiver,
        }
    }
    pub fn create_job(&mut self) -> Result<JobContext, JobError> {
        let (payload, res) = oneshot::channel();
        self.sender
            .send(JobMessage::NewJob(payload))
            .or(Err(JobError::FailedToSendToEngine))?;
        res.recv().or(Err(JobError::FailedToReceiveFromEngine))
    }

    pub fn server_link(&mut self) -> Result<LinkContext, JobError> {
        let (payload, res) = oneshot::channel();
        self.sender
            .send(JobMessage::ServerLink(payload))
            .or(Err(JobError::FailedToSendToEngine))?;
        res.recv().or(Err(JobError::FailedToReceiveFromEngine))
    }
    pub fn client_link(&mut self) -> Result<LinkContext, JobError> {
        let (payload, res) = oneshot::channel();
        self.sender
            .send(JobMessage::ClientLink(payload))
            .or(Err(JobError::FailedToSendToEngine))?;
        res.recv().or(Err(JobError::FailedToReceiveFromEngine))
    }
    pub fn connect(&mut self, node: u64, job: u64) -> Result<Stream, JobError> {
        todo!()
    }
    pub fn accept(&mut self) -> Result<(u64, u64, Stream), JobError> {
        todo!()
    }
}
