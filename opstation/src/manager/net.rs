use anyhow::{bail, Result};
use libc::{__c_anonymous_ifr_ifru, ifreq, ioctl, IFF_TUN, IFNAMSIZ};
use nix::{
    fcntl::{open, OFlag},
    sys::stat::Mode,
};
use std::{fs::File, io, os::fd::FromRawFd};

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

fn gen_key_pair() -> Result<(String, String), io::Error> {
    let output = std::process::Command::new("wg").arg("genkey").output()?;
    let private = output.stdout;
    let mut pub_key_proc = std::process::Command::new("wg")
        .arg("pubkey")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let Some(stdin) = &mut pub_key_proc.stdin else {
        return Err(io::Error::other("Failed to gen pubkey"));
    };
    stdin.write_all(&private)?;
    stdin.flush()?;
    let status = pub_key_proc.wait()?;
    if !status.success() {
        return Err(io::Error::other("Failed to gen pubkey"));
    }
    let mut stdout = pub_key_proc
        .stdout
        .take()
        .ok_or(io::Error::other("No stdout"))?;
    let mut public = Vec::new();
    stdout.read_to_end(&mut public)?;
    let private = String::from_utf8(private)
        .or(Err(io::Error::other("Failed to parse")))?
        .trim()
        .to_owned();
    let public = String::from_utf8(public)
        .or(Err(io::Error::other("Failed to parse")))?
        .trim()
        .to_owned();
    Ok((public, private))
}

pub fn create_tunnel() -> io::Result<()> {
    let status = std::process::Command::new("ip")
        .args(&["link", "add", "dev", "wg0", "type", "wireguard"])
        .status()?;
    if !status.success() {
        return Err(io::Error::other("Failed to add wg dev"));
    }
    Ok(())
}
