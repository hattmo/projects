use std::{
    collections::HashMap,
    io::Result as IoResult,
    net::{IpAddr, TcpStream, UdpSocket},
    sync::mpsc::RecvTimeoutError,
    time::Duration,
};

struct Args {}
fn main() -> IoResult<()> {
    println!("Hello, world!");
    Ok(())
}

fn listen_job() -> IoResult<()> {
    let mut buffer = [0u8; 256];
    let listen = UdpSocket::bind("0.0.0.0:1234")?;
    let broadcast = listen.try_clone()?;
    let (tx, rx) = std::sync::mpsc::channel::<()>();
    std::thread::spawn(move || loop {
        match rx.recv_timeout(Duration::from_secs(30)) {
            Err(RecvTimeoutError::Timeout) => {
                broadcast.send_to(b"hello", (std::net::Ipv4Addr::BROADCAST, 1234));
            }
            Ok(_) | Err(RecvTimeoutError::Disconnected) => break,
        };
    });
    loop {
        let (read, addr) = listen.recv_from(&mut buffer)?;
        let packet = &buffer[..read];
    }
    Ok(())
}
