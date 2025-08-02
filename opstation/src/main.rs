#![feature(linux_pidfd)]
#![feature(unix_socket_ancillary_data)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{
    io::{self, stdin},
    net::IpAddr,
};

use manager::manager_main;
use server::server_main;
use wg::WGDevice;

use crate::wg::generate_key_pair;

mod manager;
mod raw;
mod server;
mod util;
mod wg;

fn main() -> io::Result<()> {
    let mut wg_dev = WGDevice::new("wg0");
    wg_dev.set_listen_port(1234);
    let (public, private) = generate_key_pair();
    wg_dev.set_public_key(public);
    wg_dev.set_private_key(private);
    let mut peer = wg_dev.new_peer();
    peer.set_endpoint("8.8.8.8:8080");
    peer.add_allowed_ip("192.168.0.0", 24);
    let (public, _private) = generate_key_pair();
    peer.set_public_key(public);
    wg_dev.commit();

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    return Ok(());
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
