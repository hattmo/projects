use anyhow::Result;
use nix::{
    mount::{mount, MsFlags},
    sys::stat::{makedev, mknod, Mode, SFlag},
};
use std::{fs::create_dir_all, os::unix::fs::symlink, path::Path};

pub fn setup_mounts() -> Result<()> {
    create_dir_all("/dev")?;
    log::info!("created dirs");
    mount(
        Some("tmpfs"),
        "/dev",
        Some("tmpfs"),
        MsFlags::MS_NOEXEC | MsFlags::MS_STRICTATIME,
        Some("mode=755"),
    )?;
    log::info!("created tmpfs");
    create_dir_all("/dev/shm")?;
    mount(
        Some("shm"),
        "/dev/shm",
        Some("tmpfs"),
        MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
        Some("mode=1777,size=65536k"),
    )?;
    log::info!("created shm");
    create_dir_all("/dev/mqueue")?;
    mount(
        Some("mqueue"),
        "/dev/mqueue",
        Some("mqueue"),
        MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
        None::<&str>,
    )?;
    log::info!("created mqueue");
    create_dir_all("/dev/pts")?;
    mount(
        Some("devpts"),
        "/dev/pts",
        Some("devpts"),
        MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID,
        Some("newinstance,ptmxmode=0666,mode=620,gid=5"),
    )?;
    log::info!("created pts");
    create_dir_all("/sys")?;
    mount(
        Some("sysfs"),
        "/sys",
        Some("sysfs"),
        MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_RDONLY,
        None::<&str>,
    )?;
    log::info!("created sys");
    Ok(())
}

pub fn setup_devices() -> Result<()> {
    create_dir_all(Path::new("/dev/net"))?;

    let perm = Mode::from_bits(0o666).unwrap();
    mknod("/dev/null", SFlag::S_IFCHR, perm, makedev(1, 3))?;
    mknod("/dev/zero", SFlag::S_IFCHR, perm, makedev(1, 5))?;
    mknod("/dev/full", SFlag::S_IFCHR, perm, makedev(1, 7))?;
    mknod("/dev/tty", SFlag::S_IFCHR, perm, makedev(5, 0))?;
    mknod("/dev/ptmx", SFlag::S_IFCHR, perm, makedev(5, 2))?;
    mknod("/dev/random", SFlag::S_IFCHR, perm, makedev(1, 8))?;
    mknod("/dev/urandom", SFlag::S_IFCHR, perm, makedev(1, 9))?;
    mknod("/dev/net/tun", SFlag::S_IFCHR, perm, makedev(10, 200))?;

    Ok(())
}

pub fn setup_io_link() -> Result<()> {
    symlink("/proc/self/fd", "/dev/fd")?;
    symlink("/proc/self/fd/0", "/dev/stdin")?;
    symlink("/proc/self/fd/1", "/dev/stdout")?;
    symlink("/proc/self/fd/2", "/dev/stderr")?;
    Ok(())
}
