mod loop_ctrl;
use std::{
    ffi::{c_void, CStr},
    fs::{File, OpenOptions},
    io,
    os::fd::AsRawFd,
    path::{Path, PathBuf},
    ptr,
};

use libc::{c_int, c_ulong, ioctl};
use loop_ctrl::{LOOP_CLR_FD, LOOP_CTL_GET_FREE, LOOP_SET_FD};

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

pub struct LoopDev {
    loop_path: PathBuf,
}

impl LoopDev {
    pub fn new() -> io::Result<Self> {
        let lc = OpenOptions::new().write(true).open("/dev/loop-control")?;
        let open_ld = unsafe { ioctl(lc.as_raw_fd(), LOOP_CTL_GET_FREE.into()) };
        if open_ld < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self {
            loop_path: format!("/dev/loop{}", open_ld).into(),
        })
    }

    pub fn attach_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let image = OpenOptions::new().write(true).read(true).open(path)?;
        let loop_dev = OpenOptions::new().write(true).open(&self.loop_path)?;
        let set_fd_result =
            unsafe { ioctl(loop_dev.as_raw_fd(), LOOP_SET_FD.into(), image.as_raw_fd()) };
        if set_fd_result != 0 {
            return Err(io::Error::last_os_error());
        };
        Ok(())
    }
    pub fn detach_file(&self) -> io::Result<()> {
        let loop_dev = OpenOptions::new().write(true).open(&self.loop_path)?;
        let res = unsafe { ioctl(loop_dev.as_raw_fd(), LOOP_CLR_FD.into()) };
        if res != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    pub fn path(&self) -> &Path {
        &self.loop_path
    }
}
