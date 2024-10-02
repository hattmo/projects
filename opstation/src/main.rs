#![feature(linux_pidfd)]

use clap::Parser;

use manager::manager_main;
use server::server_main;

mod manager;
mod raw;
mod server;
mod util;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    manager: bool,
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    match std::env::args().nth(0) {
        Some(command) if command == "manager".to_owned() => {
            log::info!("starting manager");
            manager_main();
            log::info!("manager exited");
        }
        Some(_) => {
            log::info!("starting server");
            server_main();
            log::info!("server exited");
        }
        None => {
            log::error!("invalid arguments")
        }
    };
}
