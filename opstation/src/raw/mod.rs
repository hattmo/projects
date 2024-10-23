mod loop_ctrl;

use std::{
    ffi::{c_void, CStr},
    fs::OpenOptions,
    io, mem,
    os::fd::{AsRawFd, FromRawFd, OwnedFd},
    path::{Path, PathBuf},
    ptr,
};

use libc::{c_int, c_uint, c_ulong, ioctl, mode_t, O_RDWR};
use loop_ctrl::{
    loop_info, LOOP_CLR_FD, LOOP_CTL_GET_FREE, LOOP_GET_STATUS, LOOP_SET_FD, LOOP_SET_STATUS,
    LO_FLAGS_AUTOCLEAR,
};

pub fn mount(
    src: Option<&CStr>,
    target: &CStr,
    fstype: Option<&CStr>,
    flags: c_ulong,
    data: Option<&CStr>,
) -> io::Result<()> {
    if unsafe {
        libc::mount(
            src.map(|i| i.as_ptr()).unwrap_or(ptr::null()),
            target.as_ptr(),
            fstype.map(|i| i.as_ptr()).unwrap_or(ptr::null()),
            flags,
            data.map(|i| i.as_ptr() as *const c_void)
                .unwrap_or(ptr::null()),
        )
    } < 0
    {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

pub fn umount(target: &CStr) -> io::Result<()> {
    if unsafe { libc::umount(target.as_ptr()) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

pub fn unshare(flags: c_int) -> io::Result<()> {
    if unsafe { libc::unshare(flags) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

pub fn chroot(path: &CStr) -> io::Result<()> {
    if unsafe { libc::chroot(path.as_ptr()) } < 0 {
        return Err(io::Error::last_os_error());
    };
    Ok(())
}

pub fn mknod(path: &CStr, mode: mode_t, major: c_uint, minor: c_uint) -> io::Result<()> {
    let dev = libc::makedev(major, minor);
    unsafe { libc::mknod(path.as_ptr(), mode, dev) };
    Ok(())
}

pub struct LoopDev {
    loop_path: PathBuf,
    loop_fd: OwnedFd,
}

impl LoopDev {
    pub fn new() -> io::Result<Self> {
        let lc_fd: OwnedFd = OpenOptions::new()
            .write(true)
            .open("/dev/loop-control")?
            .into();
        let open_ld = unsafe { ioctl(lc_fd.as_raw_fd(), LOOP_CTL_GET_FREE.into()) };
        if open_ld < 0 {
            return Err(io::Error::last_os_error());
        }
        let loop_path = format!("/dev/loop{}", open_ld).into();
        let loop_fd: OwnedFd = OpenOptions::new().write(true).open(&loop_path)?.into();
        let mut loop_info: loop_info = unsafe { mem::zeroed() };
        if unsafe {
            ioctl(
                loop_fd.as_raw_fd(),
                LOOP_GET_STATUS.into(),
                ptr::from_mut(&mut loop_info),
            )
        } < 0
        {
            return Err(io::Error::last_os_error());
        };
        loop_info.lo_flags |= LO_FLAGS_AUTOCLEAR as i32;
        if unsafe {
            ioctl(
                loop_fd.as_raw_fd(),
                LOOP_SET_STATUS.into(),
                ptr::from_mut(&mut loop_info),
            )
        } < 0
        {
            return Err(io::Error::last_os_error());
        };

        Ok(Self { loop_path, loop_fd })
    }

    pub fn attach_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let image = OpenOptions::new().write(true).read(true).open(path)?;
        let set_fd_result = unsafe {
            ioctl(
                self.loop_fd.as_raw_fd(),
                LOOP_SET_FD.into(),
                image.as_raw_fd(),
            )
        };
        if set_fd_result != 0 {
            return Err(io::Error::last_os_error());
        };
        Ok(())
    }
    pub fn detach_file(&self) -> io::Result<()> {
        let res = unsafe { ioctl(self.loop_fd.as_raw_fd(), LOOP_CLR_FD.into()) };
        if res != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    pub fn path(&self) -> &Path {
        &self.loop_path
    }
}

pub fn create_pty() -> io::Result<(OwnedFd, PathBuf)> {
    let ret = unsafe { libc::posix_openpt(O_RDWR) };
    if ret < 0 {
        return Err(io::Error::last_os_error());
    }
    let master = unsafe { OwnedFd::from_raw_fd(ret) };
    if unsafe { libc::grantpt(master.as_raw_fd()) } < 0 {
        return Err(io::Error::last_os_error());
    };
    if unsafe { libc::unlockpt(master.as_raw_fd()) } < 0 {
        return Err(io::Error::last_os_error());
    };
    let mut buf = [0; 255];
    if unsafe { libc::ptsname_r(master.as_raw_fd(), buf.as_mut_ptr(), buf.len()) } < 0 {
        return Err(io::Error::last_os_error());
    };
    let slave_path: PathBuf = unsafe { CStr::from_ptr(buf.as_mut_ptr()) }
        .to_owned()
        .into_string()
        .or(Err(io::Error::other("Bad pty slave path")))?
        .into();
    Ok((master, slave_path))
}
