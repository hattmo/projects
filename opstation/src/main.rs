#![feature(linux_pidfd)]

use anyhow::Result;
use clap::Parser;

use manager::manager_main;
use server::server_main;

mod machine;
mod manager;
mod server;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    manager: bool,
}

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let arg = Args::parse();
    if arg.manager {
        log::info!("starting manager");
        manager_main()?;
        log::info!("manager exited");
    } else {
        log::info!("starting server");
        server_main()?;
        log::info!("server exited");
    }
    Ok(())
}
