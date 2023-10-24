use libc::{
    c_char, c_void, chdir, chroot, close, execve, exit, fork, getegid, geteuid, open, perror,
    strlen, unshare, waitpid, write, CLONE_NEWCGROUP, CLONE_NEWIPC, CLONE_NEWNET, CLONE_NEWNS,
    CLONE_NEWPID, CLONE_NEWUSER, CLONE_NEWUTS,
};
use std::{
    error::Error,
    ffi::{CStr, CString, NulError},
    fmt::Display,
    ptr,
};
const CLITER: for<'a> unsafe fn(&'a [u8]) -> &'a std::ffi::CStr =
    CStr::from_bytes_with_nul_unchecked;

// const STACK_SIZE: usize = 10 * 1024 * 1024;

#[derive(Debug)]
enum ContainerError {
    ForkFailed,
    ArgContainedNul,
    WaitFailed,
}

impl Display for ContainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerError::ForkFailed => write!(f, "fork failed"),
            ContainerError::ArgContainedNul => write!(f, "arg contained nul byte"),
            ContainerError::WaitFailed => write!(f, "wait failed"),
        }
    }
}

impl Error for ContainerError {}

fn main() {
    match create_container("/bin/bash", &[]) {
        Ok(ret) => {
            print!("ret: {}", ret)
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    };
}

#[no_mangle]
fn create_container(cmd: &str, args: &[&str]) -> Result<i32, ContainerError> {
    // let cmd = CString::new(cmd.as_bytes()).map_err(|_| ContainerError::ArgContainedNul)?;
    let args_buffers: Result<Vec<CString>, NulError> = [cmd]
        .iter()
        .chain(args.iter())
        .map(|arg| CString::new(arg.as_bytes()))
        .collect();
    let args_buffers = args_buffers.map_err(|_| ContainerError::ArgContainedNul)?;
    let args: Vec<*const c_char> = args_buffers
        .iter()
        .map(|arg| arg.as_ptr())
        .chain([std::ptr::null()])
        .collect();
    unsafe {
        let uid_map_buf: CString =
            CString::new(format!("0 {} 1\n", geteuid())).expect("Get uid cant have a null byte");
        let uid_map = uid_map_buf.as_ptr();
        let uid_map_len = strlen(uid_map);

        let gid_map_buf: CString =
            CString::new(format!("0 {} 1\n", getegid())).expect("Get gid cant have a null byte");
        let gid_map = gid_map_buf.as_ptr();
        let gid_map_len = strlen(gid_map);

        match fork() {
            err_code if err_code < 0 => Err(ContainerError::ForkFailed),
            pid if pid > 0 => {
                let mut status: i32 = 0;
                let res = waitpid(pid, &mut status, 0);
                if res < 0 {
                    return Err(ContainerError::WaitFailed);
                }
                Ok(status)
            }
            _ => {
                // child only async-signal-safe code from here on
                unshare(
                    CLONE_NEWUSER
                        | CLONE_NEWNS
                        | CLONE_NEWPID
                        | CLONE_NEWIPC
                        | CLONE_NEWUTS
                        | CLONE_NEWCGROUP,
                    // | CLONE_NEWNET
                );
                let uid_map_fd = open(
                    CLITER(b"/proc/self/uid_map\0").as_ptr(),
                    libc::O_WRONLY,
                    0o644,
                );
                let written = write(uid_map_fd, uid_map as *const c_void, uid_map_len);
                close(uid_map_fd);
                if written < uid_map_len.try_into().unwrap() {
                    perror(CLITER(b"write uid\0").as_ptr());
                    exit(1)
                }

                let setgroups_fd = open(
                    CLITER(b"/proc/self/setgroups\0").as_ptr(),
                    libc::O_WRONLY,
                    0o644,
                );
                let written = write(setgroups_fd, CLITER(b"deny").as_ptr() as *const c_void, 4);
                if written < 4 {
                    perror(CLITER(b"write setgroups\0").as_ptr());
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
                    perror(CLITER(b"write guid\0").as_ptr());
                    exit(1)
                }

                chdir(CLITER(b"./ubuntu\0").as_ptr());
                let ret = chroot(CLITER(b"./\0").as_ptr());
                if ret < 0 {
                    perror(CLITER(b"chroot\0").as_ptr());
                    exit(1)
                }

                match fork() {
                    err_code if err_code < 0 => Err(ContainerError::ForkFailed),
                    pid if pid > 0 => {
                        let mut status: i32 = 0;
                        let res = waitpid(pid, &mut status, 0);
                        if res < 0 {
                            return Err(ContainerError::WaitFailed);
                        }
                        Ok(status)
                    }
                    _ => {
                        // child only async-signal-safe code from here on
                        execve(args[0], args.as_ptr(), ptr::null());
                        exit(1);
                    }
                }
            }
        }
    }
}
