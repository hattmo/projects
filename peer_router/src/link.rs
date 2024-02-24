use crate::{EngineMessage, Message};
use rustls::Connection;
use std::{
    collections::VecDeque,
    io::{Read, Write},
    sync::mpsc::{Receiver, Sender, TryRecvError},
};

pub(crate) struct Link {
    pub sender: Sender<EngineMessage>,
    pub receiver: Receiver<LinkMessage>,
}

pub struct LinkContext {
    conn: Connection,
    sender: Sender<LinkMessage>,
    receiver: Receiver<EngineMessage>,
    buffer: VecDeque<u8>,
}

pub enum LinkError {
    FailedToSendToEngine,
    FailedToReceiveFromEngine,
}

pub enum LinkMessage {
    Kill,
    Message(Message),
}

impl LinkContext {
    pub(crate) fn new(
        sender: Sender<LinkMessage>,
        receiver: Receiver<EngineMessage>,
        conn: Connection,
    ) -> LinkContext {
        LinkContext {
            conn,
            sender,
            receiver,
            buffer: VecDeque::new(),
        }
    }
}

impl Read for LinkContext {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.receiver.try_recv() {
            Ok(EngineMessage::Message(mut chunk)) => {
                self.conn.writer().write_all(&mut chunk.payload)?;
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                return Err(std::io::Error::other("Engine disconnected"));
            }
        }
        self.conn.write_tls(&mut self.buffer)?;
        self.buffer.read(buf)
    }
}
impl Write for LinkContext {
    fn write(&mut self, mut buf: &[u8]) -> std::io::Result<usize> {
        let consumed = self.conn.read_tls(&mut buf)?;
        let mut chunk = Vec::new();
        self.conn.reader().read_to_end(&mut chunk);
        self.sender.send(LinkMessage::Message(Message {
            from: crate::Addr { node: 0, job: 0 },
            to: crate::Addr { node: 0, job: 0 },
            stream: 0,
            seq: 0,
            ack: 0,
            payload: vec![1, 2, 3],
        }));
        Ok(consumed)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
