pub mod mount;
pub mod net;

use anyhow::{bail, Result};
use cstr::cstr;
use libc::{
    mount, unshare, CLONE_NEWIPC, CLONE_NEWNET, CLONE_NEWNS, CLONE_NEWPID, CLONE_NEWUTS, MS_NODEV,
    MS_NOEXEC, MS_NOSUID,
};
use mount::{setup_devices, setup_io_link, setup_mounts};
use net::create_tun;
use nix::unistd::chroot;
use std::io::Error;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::ptr;
use std::{env::current_dir, fs::create_dir_all};
pub fn manager_main() -> Result<()> {
    unsafe {
        const NS_FLAGS: i32 =
            CLONE_NEWNS | CLONE_NEWPID | CLONE_NEWIPC | CLONE_NEWUTS | CLONE_NEWNET;
        let res = unshare(NS_FLAGS);
        if res < 0 {
            log::error!("unshare failed");
            bail!("unshare failed")
        }
        log::info!("unshared");
        chroot(&current_dir()?)?;
        log::info!("chrooted");
        setup_mounts()?;
        log::info!("mounted");
        setup_devices()?;
        log::info!("devices");
        setup_io_link()?;
        log::info!("io link");
        let _tun = create_tun()?;
        log::info!("tun");
    }
    log::info!("starting init");
    let mut child = unsafe {
        std::process::Command::new("/bin/bash")
            .env_clear()
            .pre_exec(|| {
                create_dir_all(Path::new("/proc")).map_err(Error::other)?;
                let res = mount(
                    cstr!("proc").as_ptr(),
                    cstr!("/proc").as_ptr(),
                    cstr!("proc").as_ptr(),
                    MS_NOEXEC | MS_NOSUID | MS_NODEV,
                    ptr::null(),
                );
                if res < 0 {
                    log::error!("mount proc failed");
                    return Err(Error::last_os_error());
                }
                Ok(())
            })
            .spawn()?
    };
    log::info!("init started");
    child.wait()?;
    log::info!("init exited");
    Ok(())
}
