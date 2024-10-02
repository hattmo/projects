pub mod net;
use crate::raw::{chroot, mount, unshare};
use crate::util::AsCString;
use libc::{
    makedev, mknod, mode_t, CLONE_NEWIPC, CLONE_NEWNET, CLONE_NEWNS, CLONE_NEWPID, CLONE_NEWUTS,
    MS_NODEV, MS_NOEXEC, MS_NOSUID, MS_RDONLY, MS_STRICTATIME, S_IFCHR, S_IRGRP, S_IROTH, S_IRUSR,
    S_IWGRP, S_IWOTH, S_IWUSR,
};
use std::{
    env::current_dir,
    fs::create_dir_all,
    io,
    os::unix::{fs::symlink, process::CommandExt},
    path::Path,
};

pub fn manager_main() -> io::Result<()> {
    const NS_FLAGS: i32 = CLONE_NEWNS | CLONE_NEWPID | CLONE_NEWIPC | CLONE_NEWUTS | CLONE_NEWNET;
    unshare(NS_FLAGS)?;
    log::info!("unshared");
    chroot(&current_dir()?.as_cstring()?)?;
    log::info!("chrooted");
    setup_mounts()?;
    log::info!("mounted");
    setup_devices()?;
    log::info!("devices");
    setup_io_link()?;
    log::info!("io link");
    log::info!("starting init");
    let mut child = unsafe {
        std::process::Command::new("/bin/bash")
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
                Ok(())
            })
    }
    .spawn()?;
    log::info!("init started");
    child.wait()?;
    log::info!("init exited");
    Ok(())
}

pub fn setup_mounts() -> io::Result<()> {
    create_dir_all("/dev")?;
    log::info!("created dirs");
    mount(
        Some(c"tmpfs"),
        c"/dev",
        Some(c"tmpfs"),
        MS_NOEXEC | MS_STRICTATIME,
        Some(c"mode=755"),
    )?;
    log::info!("created tmpfs");
    create_dir_all("/dev/shm")?;
    mount(
        Some(c"shm"),
        c"/dev/shm",
        Some(c"tmpfs"),
        MS_NOEXEC | MS_NOSUID | MS_NODEV,
        Some(c"mode=1777,size=65536k"),
    )?;

    log::info!("created shm");
    create_dir_all("/dev/mqueue")?;
    mount(
        Some(c"mqueue"),
        c"/dev/mqueue",
        Some(c"mqueue"),
        MS_NOEXEC | MS_NOSUID | MS_NODEV,
        None,
    )?;
    log::info!("created mqueue");
    create_dir_all("/dev/pts")?;
    mount(
        Some(c"devpts"),
        c"/dev/pts",
        Some(c"devpts"),
        MS_NOEXEC | MS_NOSUID,
        Some(c"newinstance,ptmxmode=0666,mode=620,gid=5"),
    )?;
    log::info!("created pts");
    create_dir_all("/sys")?;
    mount(
        Some(c"sysfs"),
        c"/sys",
        Some(c"sysfs"),
        MS_NOEXEC | MS_NOSUID | MS_NODEV | MS_RDONLY,
        None,
    )?;
    log::info!("created sys");
    Ok(())
}

pub fn setup_devices() -> io::Result<()> {
    create_dir_all(Path::new("/dev/net"))?;

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

pub fn setup_io_link() -> io::Result<()> {
    symlink("/proc/self/fd", "/dev/fd")?;
    symlink("/proc/self/fd/0", "/dev/stdin")?;
    symlink("/proc/self/fd/1", "/dev/stdout")?;
    symlink("/proc/self/fd/2", "/dev/stderr")?;
    Ok(())
}
