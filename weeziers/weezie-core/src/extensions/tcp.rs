use bincode::de::read::IoReader;

use crate::{messages::Message, CapID, Extension};

use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

pub struct TCPConnectTunnel {
    handle: JoinHandle<Result<(), std::io::Error>>,
    send: Sender<Vec<Message>>,
    recv: Receiver<Vec<Message>>,
}

impl TCPConnectTunnel {
    pub fn new(addr: SocketAddr, sleep_time: Duration) -> Self {
        let (send_up, recv_down) = mpsc::channel::<Vec<Message>>();
        let (send_down, recv_up) = mpsc::channel::<Vec<Message>>();
        Self {
            handle: spawn(move || thread_worker(addr, sleep_time, send_up, recv_up)),
            send: send_down,
            recv: recv_down,
        }
    }
}
impl Extension for TCPConnectTunnel {
    fn init(&self) -> CapID {
        1
    }

    fn exec(&mut self, message: &mut Vec<crate::Message>) -> Vec<crate::Message> {
        todo!()
    }
}

fn thread_worker(
    addr: SocketAddr,
    sleep_time: Duration,
    send: Sender<Vec<Message>>,
    recv: Receiver<Vec<Message>>,
) -> Result<(), io::Error> {
    loop {
        sleep(sleep_time);

        let conn = TcpStream::connect(&addr)?;
        conn.set_read_timeout(Some(Duration::from_secs(2)))?;
        conn.set_write_timeout(Some(Duration::from_secs(2)))?;

        let outbound_mess: Vec<Message> = recv.try_recv().unwrap_or_default();
        bincode::serialize_into(conn.try_clone()?, &outbound_mess).unwrap_or_default();

        let inbound_mess = bincode::deserialize_from(conn).unwrap_or_default();
        if send.send(inbound_mess).is_err() {
            break;
        };
    }
    Ok(())
}
