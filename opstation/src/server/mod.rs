use std::{
    fs::File,
    io,
    os::{
        fd::OwnedFd,
        unix::{net::UnixDatagram, process::CommandExt},
    },
};

use libc::CLONE_NEWNS;

use crate::{
    raw::{mount, unshare},
    util::ReceiveAncillary,
};

pub fn server_main() -> io::Result<()> {
    unshare(CLONE_NEWNS)?;
    std::fs::create_dir_all("/tmp/opstation/")?;
    mount(
        Some(c"tmpfs"),
        c"/tmp/opstation/",
        Some(c"tmpfs"),
        0,
        Some(c"mode=755"),
    )?;
    let (parent, child) = UnixDatagram::pair()?;
    let child: OwnedFd = child.into();
    let mut child = std::process::Command::new(std::env::current_exe()?)
        .stdin(child)
        .arg0("manager")
        .arg("./arch.img")
        .arg("")
        .spawn()?;
    log::info!("waiting for pty");
    let mut data_buffer = [0; 255];
    let (_, ancillary) = parent.recv_ancillary(&mut data_buffer)?;
    let mut pty_master: File = ancillary
        .into_iter()
        .next()
        .ok_or(io::Error::other("No fd in message"))?
        .into();
    std::io::copy(&mut pty_master, &mut std::io::stdout())?;
    log::info!("pty closed");
    child.wait()?;
    Ok(())
}
