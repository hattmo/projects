#![feature(linux_pidfd)]
#![feature(unix_socket_ancillary_data)]

use std::{
    io::{self, stdin},
    net::Ipv4Addr,
};

use manager::manager_main;
use server::server_main;
use wg::{generate_key_pair, WGDevice};

mod manager;
mod nl;
mod raw;
mod server;
mod util;
mod wg;

fn main() -> io::Result<()> {
    unsafe { nl::make_vx_lan() };
    return Ok(());
    let mut wg_dev = WGDevice::new("wg0");
    wg_dev.set_listen_port(1234);
    let (public, private) = generate_key_pair();
    wg_dev.set_public_key(public);
    wg_dev.set_private_key(private);

    for addr in (Ipv4Addr::new(192, 168, 0, 0)..Ipv4Addr::new(192, 168, 0, 255))
        .into_iter()
        .step_by(4)
    {
        let mut peer = wg_dev.new_peer();
        peer.set_endpoint("192.168.0.1:8080");
        peer.add_allowed_ip(addr.into(), 31);
        let (public, _private) = generate_key_pair();
        peer.set_public_key(public);
    }
    wg_dev.commit();

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    // testing
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    match std::env::args().next() {
        Some(command) if command == "manager".to_owned() => {
            log::info!("starting manager");
            manager_main()?;
            log::info!("manager exited");
        }
        Some(_) => {
            log::info!("starting server");
            server_main()?;
            log::info!("server exited");
        }
        None => {
            log::error!("invalid arguments")
        }
    };
    Ok(())
}
