#![feature(lazy_cell)]
mod raw;

use anyhow::{bail, Result};
use libc::{
    c_char, chdir, chroot, execve, exit, fork, unshare, waitpid, CLONE_NEWIPC, CLONE_NEWNET,
    CLONE_NEWNS, CLONE_NEWPID, CLONE_NEWUTS,
};
use std::{
    ffi::{CString, NulError},
    ptr,
};

pub fn create_container() -> Result<i32> {
    let args_buffers: Vec<CString> = [cmd]
        .iter()
        .chain(args.iter())
        .map(|arg| CString::new(arg.as_bytes()))
        .collect::<Result<_, NulError>>()?;
    let args: Vec<*const c_char> = args_buffers
        .iter()
        .map(|arg| arg.as_ptr())
        .chain([std::ptr::null()])
        .collect();

    unsafe {
        match fork() {
            err_code if err_code < 0 => bail!("fork failed"),
            child_pid if child_pid > 0 => {
                let mut grandchild_pid = 0;
                let res = waitpid(child_pid, &mut grandchild_pid, 0);
                if res < 0 || grandchild_pid < 0 {
                    bail!("waitpid failed")
                }
                Ok(grandchild_pid)
            }
            0 => {
                const NS_FLAGS: i32 =
                    CLONE_NEWNS | CLONE_NEWPID | CLONE_NEWIPC | CLONE_NEWUTS | CLONE_NEWNET;
                unshare(NS_FLAGS);
                match fork() {
                    err_code if err_code < 0 => exit(-1),
                    grandchild_pid if grandchild_pid > 0 => {
                        let mut status = 0;
                        waitpid(grandchild_pid, &mut status, 0);
                        exit(0);
                    }
                    0 => {
                        setup_root();
                        setup_mounts();
                        setup_devices();
                        setup_io_link();
                        execve(args[0], args.as_ptr(), ptr::null());
                        exit(0);
                    }
                }
            }
        }
    }
}

fn setup_root() -> Result<()> {
    unsafe {
        let ret = chdir(c"./merged".as_ptr());
        if ret < 0 {
            bail!("chdir failed")
        };
        let ret = chroot(c"./".as_ptr());
        if ret < 0 {
            bail!("chroot failed")
        };
        Ok(())
    }
}
