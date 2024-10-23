mod net;
mod rootfs;

use crate::raw::{chroot, create_pty, mount, unshare};
use crate::util::{AsCString, SendAncillary};
use clap::Parser;
use libc::{
    makedev, mknod, mode_t, CLONE_NEWIPC, CLONE_NEWNET, CLONE_NEWNS, CLONE_NEWPID, CLONE_NEWUTS,
    MS_NODEV, MS_NOEXEC, MS_NOSUID, MS_RDONLY, MS_STRICTATIME, S_IFCHR, S_IRGRP, S_IROTH, S_IRUSR,
    S_IWGRP, S_IWOTH, S_IWUSR,
};
use rootfs::RootFS;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::net::UnixDatagram;
use std::{
    fs::create_dir_all,
    io,
    os::unix::{fs::symlink, process::CommandExt},
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    image: String,
    #[arg(short, long)]
    ptys: usize,
}

pub fn manager_main() -> io::Result<()> {
    let mut to_server = unsafe { UnixDatagram::from_raw_fd(std::io::stdin().as_raw_fd()) };
    let args = Args::parse();
    // We should expect the server to set us up with a unix socket for stdin
    unshare(CLONE_NEWNS | CLONE_NEWPID | CLONE_NEWIPC | CLONE_NEWUTS | CLONE_NEWNET)?;
    let root = RootFS::new("./arch.img")?;
    chroot(&root.path().as_cstring())?;
    setup_mounts()?;
    setup_devices()?;
    let (masters, slaves) = setup_ptys(args.ptys)?;
    println!("{slaves:?}");
    let buf = [0];
    to_server.send_ancillary(&buf, &masters)?;

    setup_links()?;
    log::info!("starting init");
    let mut child = unsafe {
        std::process::Command::new("/sbin/init")
            .env_clear()
            .pre_exec(|| {
                create_dir_all(Path::new("/proc")).map_err(io::Error::other)?;
                mount(
                    Some(c"proc"),
                    c"/proc",
                    Some(c"proc"),
                    MS_NOEXEC | MS_NOSUID | MS_NODEV,
                    None,
                )?;
                log::info!("spawning");
                Ok(())
            })
    }
    .spawn()?;
    log::info!("init started");
    child.wait()?;
    log::info!("init exited");
    Ok(())
}

fn setup_ptys(num: usize) -> io::Result<(Vec<OwnedFd>, Vec<PathBuf>)> {
    (0..num).into_iter().map(|_| create_pty()).collect()
}

pub fn setup_mounts() -> io::Result<()> {
    create_dir_all("/dev")?;
    mount(
        Some(c"tmpfs"),
        c"/dev",
        Some(c"tmpfs"),
        MS_NOEXEC | MS_STRICTATIME,
        Some(c"mode=755"),
    )?;
    log::info!("created /dev");
    create_dir_all("/sys")?;
    mount(
        Some(c"sysfs"),
        c"/sys",
        Some(c"sysfs"),
        MS_NOEXEC | MS_NOSUID | MS_NODEV | MS_RDONLY,
        None,
    )?;
    log::info!("created /sys");
    create_dir_all("/dev/pts")?;
    mount(
        Some(c"devpts"),
        c"/dev/pts",
        Some(c"devpts"),
        MS_NOEXEC | MS_NOSUID,
        Some(c"newinstance,ptmxmode=0666,mode=620,gid=5"),
    )?;
    log::info!("created /dev/pts");
    Ok(())
}

pub fn setup_devices() -> io::Result<()> {
    create_dir_all("/dev/net")?;
    create_dir_all("dev/pts")?;
    let mode: mode_t = S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH;
    unsafe {
        mknod(c"/dev/null".as_ptr(), S_IFCHR | mode, makedev(1, 3));
        mknod(c"/dev/zero".as_ptr(), S_IFCHR | mode, makedev(1, 5));
        mknod(c"/dev/full".as_ptr(), S_IFCHR | mode, makedev(1, 7));
        mknod(c"/dev/random".as_ptr(), S_IFCHR | mode, makedev(1, 8));
        mknod(c"/dev/urandom".as_ptr(), S_IFCHR | mode, makedev(1, 9));
        mknod(c"/dev/tty".as_ptr(), S_IFCHR | mode, makedev(5, 0));
        mknod(c"/dev/ptmx".as_ptr(), S_IFCHR | mode, makedev(5, 2));
    }

    Ok(())
}

pub fn setup_links() -> io::Result<()> {
    symlink("/proc/self/fd", "/dev/fd")?;
    symlink("/proc/self/fd/0", "/dev/stdin")?;
    symlink("/proc/self/fd/1", "/dev/stdout")?;
    symlink("/proc/self/fd/2", "/dev/stderr")?;
    Ok(())
}
