use std::{
    os::{
        fd::{FromRawFd, OwnedFd},
        unix::net::UnixListener,
    },
    thread,
};

use clap::Parser;
/// A helper program to execute a command for each connection
#[derive(Parser)]
struct Arguments {
    /// The command to execute
    command: String,
}
fn main() {
    let Arguments { command } = Arguments::parse();
    let owned = unsafe { OwnedFd::from_raw_fd(0) };
    let lisener: UnixListener = owned.into();
    for conn in lisener.into_iter() {
        let Ok(conn) = conn else {
            println!("Error accepting connection");
            break;
        };
        let command = command.clone();
        thread::spawn(move || -> Result<(), &'static str> {
            let std_in: OwnedFd = conn.into();
            let std_out = std_in.try_clone().map_err(|_| "Failed to clone")?;
            std::process::Command::new("/bin/sh")
                .arg("-c")
                .arg(command)
                .stdin(std_in)
                .stdout(std_out)
                .status()
                .map_err(|_| "Failed to execute")?;
            Ok(())
        });
    }
}
