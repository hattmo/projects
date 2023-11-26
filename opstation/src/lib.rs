use libc::{
    __c_anonymous_ifr_ifru, c_char, c_void, chdir, chroot, close, execve, exit, fork, getegid,
    geteuid, ifreq, mount, open, strlen, unshare, waitpid, write, CLONE_NEWCGROUP, CLONE_NEWIPC,
    CLONE_NEWNET, CLONE_NEWNS, CLONE_NEWPID, CLONE_NEWUSER, CLONE_NEWUTS, IFF_TUN, IFNAMSIZ,
    MS_NODEV, MS_NOEXEC, MS_NOSUID, MS_RELATIME, O_RDWR, ioctl, c_int,
};
use std::{
    error::Error,
    ffi::{CStr, CString, NulError},
    fmt::Display,
    ptr,
};

#[derive(Debug)]
pub enum ContainerError {
    ForkFailed,
    ArgContainedNul(NulError),
    WaitFailed,
}

impl Display for ContainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerError::ForkFailed => write!(f, "fork failed"),
            ContainerError::ArgContainedNul(_e) => write!(f, "arg contained nul byte"),
            ContainerError::WaitFailed => write!(f, "wait failed"),
        }
    }
}

impl Error for ContainerError {}

const CLITER: for<'a> unsafe fn(&'a [u8]) -> &'a std::ffi::CStr =
    CStr::from_bytes_with_nul_unchecked;

pub fn create_container(cmd: &str, args: &[&str], _image: &str) -> Result<i32, ContainerError> {
    let args_buffers: Vec<CString> = [cmd]
        .iter()
        .chain(args.iter())
        .map(|arg| CString::new(arg.as_bytes()))
        .collect::<Result<_, NulError>>()
        .map_err(ContainerError::ArgContainedNul)?;
    let args: Vec<*const c_char> = args_buffers
        .iter()
        .map(|arg| arg.as_ptr())
        .chain([std::ptr::null()])
        .collect();

    unsafe {
        let uid = geteuid();
        let gid = getegid();
        let uid_map_buf: CString =
            CString::new(format!("0 {uid} 1\n")).map_err(ContainerError::ArgContainedNul)?;
        let uid_map = uid_map_buf.as_ptr();
        let gid_map_buf: CString =
            CString::new(format!("0 {gid} 1\n")).map_err(ContainerError::ArgContainedNul)?;
        let gid_map = gid_map_buf.as_ptr();

        match fork() {
            err_code if err_code < 0 => Err(ContainerError::ForkFailed),
            child_pid if child_pid > 0 => {
                let mut gandchild_pid = 0;
                let res = waitpid(child_pid, &mut gandchild_pid, 0);
                if res < 0 || gandchild_pid < 0 {
                    return Err(ContainerError::WaitFailed);
                }
                Ok(gandchild_pid)
            }
            _ => {
                // child only async-signal-safe code from here on
                let mut flags = CLONE_NEWNS
                    | CLONE_NEWPID
                    | CLONE_NEWIPC
                    | CLONE_NEWUTS
                    | CLONE_NEWCGROUP
                    | CLONE_NEWNET;
                if uid != 0 || gid != 0 {
                    flags |= CLONE_NEWUSER;
                }
                unshare(flags);
                if uid != 0 || gid != 0 {
                    write_maps(uid_map, gid_map);
                }
                create_tun();
                match fork() {
                    err_code if err_code < 0 => exit(-1),
                    grandchild_pid if grandchild_pid > 0 => {
                        let mut status = 0;
                        waitpid(grandchild_pid, &mut status, 0);
                        exit(0);
                    }
                    _ => {
                        jail();
                        setup_mounts();
                        // child only async-signal-safe code from here on
                        execve(args[0], args.as_ptr(), ptr::null());
                        exit(0);
                    }
                }
            }
        }
    }
}

unsafe fn create_tun() -> c_int{
    let fd = open(CLITER(b"/dev/net/tun\0").as_ptr(), O_RDWR);
    if fd < 0 {
        exit(-1)
    };
    let mut name = [0i8; IFNAMSIZ];
    "tun0"
        .as_bytes()
        .iter()
        .enumerate()
        .for_each(|(i, &b)| name[i] = b as i8);
    let ifreq = ifreq {
        ifr_name: name,
        ifr_ifru: __c_anonymous_ifr_ifru {
            ifru_flags: IFF_TUN as i16,
        },
    };
    ioctl(fd, 0x400454ca, &ifreq);
    fd
}

unsafe fn jail() {
    let ret = chdir(CLITER(b"./ubuntu\0").as_ptr());
    if ret < 0 {
        exit(-1)
    };
    let ret = chroot(CLITER(b"./\0").as_ptr());
    if ret < 0 {
        exit(-1)
    };
}

unsafe fn setup_mounts() {
    mount(
        CLITER(b"proc\0").as_ptr(),
        CLITER(b"/proc\0").as_ptr(),
        CLITER(b"proc\0").as_ptr(),
        MS_NOSUID | MS_NODEV | MS_NOEXEC | MS_RELATIME,
        ptr::null(),
    );
}

unsafe fn write_maps(uid_map: *const c_char, gid_map: *const c_char) {
    let uid_map_len = strlen(uid_map);
    let gid_map_len = strlen(gid_map);
    let uid_map_fd = open(
        CLITER(b"/proc/self/uid_map\0").as_ptr(),
        libc::O_WRONLY,
        0o644,
    );
    let written = write(uid_map_fd, uid_map as *const c_void, uid_map_len);
    close(uid_map_fd);
    if written < uid_map_len.try_into().unwrap() {
        exit(1)
    }

    let setgroups_fd = open(
        CLITER(b"/proc/self/setgroups\0").as_ptr(),
        libc::O_WRONLY,
        0o644,
    );
    let written = write(setgroups_fd, CLITER(b"deny").as_ptr() as *const c_void, 4);
    if written < 4 {
        exit(1)
    }
    let gid_map_fd = open(
        CLITER(b"/proc/self/gid_map\0").as_ptr(),
        libc::O_WRONLY,
        0o644,
    );
    let written = write(gid_map_fd, gid_map as *const c_void, gid_map_len);
    close(gid_map_fd);
    if written < gid_map_len.try_into().unwrap() {
        exit(1)
    }
}
