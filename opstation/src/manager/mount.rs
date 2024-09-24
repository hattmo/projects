use anyhow::Result;
use libc::{
    dev_t, makedev, mknod, mount, MS_NOEXEC, MS_STRICTATIME, S_IFCHR, S_IRGRP, S_IROTH, S_IRUSR,
    S_IWGRP, S_IWOTH, S_IWUSR,
};
use std::{
    fs::create_dir_all,
    mem,
    os::unix::{fs::symlink, raw::mode_t},
    path::Path,
};

pub fn setup_mounts() -> Result<()> {
    create_dir_all("/dev")?;
    log::info!("created dirs");
    unsafe {
        mount(
            c"tmpfs",
            c"/dev",
            "tmpfs",
            MS_NOEXEC | MS_STRICTATIME,
            c"mode=755",
        )
    };
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

    let mode: mode_t = S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH;
    unsafe {
        mknod(c"/dev/null", S_IFCHR | mode, makedev(1, 3));
        mknod(c"/dev/zero", S_IFCHR | mode, makedev(1, 5));
        mknod(c"/dev/full", S_IFCHR | mode, makedev(1, 7));
        mknod(c"/dev/random", S_IFCHR | mode, makedev(1, 8));
        mknod(c"/dev/urandom", S_IFCHR | mode, makedev(1, 9));
        mknod(c"/dev/tty", S_IFCHR | mode, makedev(5, 0));
        mknod(c"/dev/ptmx", S_IFCHR | mode, makedev(5, 2));
    }

    Ok(())
}

pub fn setup_io_link() -> Result<()> {
    symlink("/proc/self/fd", "/dev/fd")?;
    symlink("/proc/self/fd/0", "/dev/stdin")?;
    symlink("/proc/self/fd/1", "/dev/stdout")?;
    symlink("/proc/self/fd/2", "/dev/stderr")?;
    Ok(())
}
