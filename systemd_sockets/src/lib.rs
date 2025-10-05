use std::{net::TcpListener, os::unix::net::UnixListener};

struct SystemdSockets {
    max: usize,
    on: usize,
}

enum Listener {
    Tcp(TcpListener),
    Unix(UnixListener),
}

enum SystemdSocket {
    Datagram(SocketDatagram),
    Stream(Listener),
    SeqPacket,
}

impl Iterator for SystemdSockets {
    type Item = (Option<String>, SystemdSocket);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub fn get_sockets() -> Result<SystemdSockets, ()> {
    std::env::var("LISTEN_FD");
    Err(())
}
