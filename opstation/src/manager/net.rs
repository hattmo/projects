use anyhow::{bail, Result};
use libc::{__c_anonymous_ifr_ifru, ifreq, ioctl, IFF_TUN, IFNAMSIZ};
use nix::{
    fcntl::{open, OFlag},
    sys::stat::Mode,
};
use std::{fs::File, os::fd::FromRawFd};

pub fn create_tun() -> Result<File> {
    // let tun = OpenOptions::new()
    //     .write(true)
    //     .read(true)
    //     .open("/dev/net/tun")?;
    let fd = open(
        "/dev/net/tun",
        nix::fcntl::OFlag::O_CLOEXEC | OFlag::O_RDWR,
        Mode::empty(),
    )?;
    let mut name = [0i8; IFNAMSIZ];
    b"eth0"
        .iter()
        .enumerate()
        .for_each(|(i, &b)| name[i] = b as i8);
    let ifreq = ifreq {
        ifr_name: name,
        ifr_ifru: __c_anonymous_ifr_ifru {
            ifru_flags: IFF_TUN as i16,
        },
    };
    unsafe {
        let ret = ioctl(fd, 0x400454ca, &ifreq);
        if ret < 0 {
            bail!("failed to ioctl tun")
        }
        Ok(File::from_raw_fd(fd))
    }
}
