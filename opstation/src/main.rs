#![feature(linux_pidfd)]
#![feature(unix_socket_ancillary_data)]
use std::io;

use manager::manager_main;
use server::server_main;

mod manager;
mod raw;
mod server;
mod util;

fn main() -> io::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    match std::env::args().nth(0) {
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
