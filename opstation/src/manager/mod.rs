mod net;
mod rootfs;

use crate::raw::{chroot, mount, unshare};
use crate::util::AsCString;
use clap::Parser;
use libc::{
    makedev, mknod, mode_t, CLONE_NEWIPC, CLONE_NEWNET, CLONE_NEWNS, CLONE_NEWPID, CLONE_NEWUTS,
    MS_BIND, MS_NODEV, MS_NOEXEC, MS_NOSUID, MS_RDONLY, MS_STRICTATIME, S_IFCHR, S_IRGRP, S_IROTH,
    S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR,
};
use rootfs::RootFS;
use std::{
    fs::create_dir_all,
    io,
    os::unix::{fs::symlink, process::CommandExt},
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
struct Args {
    image: String,
    pty: Vec<String>,
}

pub fn manager_main() -> io::Result<()> {
    let args = Args::parse();
    // We should expect the server to set us up with a unix socket for stdin
    unshare(CLONE_NEWNS | CLONE_NEWPID | CLONE_NEWIPC | CLONE_NEWUTS | CLONE_NEWNET)?;
    let root = RootFS::new("./arch.img")?;
    setup_pty(root.path(), &args.pty)?;
    setup_mounts(root.path())?;
    chroot(&root.path().as_cstring())?;
    setup_devices()?;
    setup_io_link()?;
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

fn setup_pty(root: &Path, ptys: &[String]) -> io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for (i, pty) in ptys.into_iter().enumerate() {
        let src_path = Path::new(pty);
        let dst_path = root.join(format!("dev/pts/{i}"));
        mount(
            Some(&src_path.as_cstring()),
            &dst_path.as_cstring(),
            None,
            MS_BIND,
            None,
        )?;
        out.push(format!("pts/{i}").into())
    }
    Ok(out)
}

pub fn setup_mounts(root: &Path) -> io::Result<()> {
    create_dir_all(root.join("dev"))?;
    log::info!("created dirs");
    mount(
        Some(c"tmpfs"),
        c"/dev",
        Some(c"tmpfs"),
        MS_NOEXEC | MS_STRICTATIME,
        Some(c"mode=755"),
    )?;
    log::info!("created tmpfs");
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

pub fn setup_io_link() -> io::Result<()> {
    symlink("/proc/self/fd", "/dev/fd")?;
    symlink("/proc/self/fd/0", "/dev/stdin")?;
    symlink("/proc/self/fd/1", "/dev/stdout")?;
    symlink("/proc/self/fd/2", "/dev/stderr")?;
    Ok(())
}
