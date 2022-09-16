pub mod namespace;
use anyhow::{bail, Result};
use cstr::cstr;
use libc::{
    c_char, chdir, chroot, execve, exit, fork, unshare, waitpid, CLONE_NEWIPC, CLONE_NEWNET,
    CLONE_NEWNS, CLONE_NEWPID, CLONE_NEWUTS,
};

use namespace::{mount::setup_mounts, net::create_tun};
use std::{
    ffi::{CString, NulError},
    ptr,
};

use crate::namespace::mount::{setup_devices, setup_io_link};

pub fn create_container(cmd: &str, args: &[&str], _image: &str) -> Result<i32> {
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
                let mut gandchild_pid = 0;
                let res = waitpid(child_pid, &mut gandchild_pid, 0);
                if res < 0 || gandchild_pid < 0 {
                    bail!("waitpid failed")
                }
                Ok(gandchild_pid)
            }
            _ => {
                const NS_FLAGS: i32 =
                    CLONE_NEWNS | CLONE_NEWPID | CLONE_NEWIPC | CLONE_NEWUTS | CLONE_NEWNET;
                unshare(NS_FLAGS);
                create_tun();
                match fork() {
                    err_code if err_code < 0 => exit(-1),
                    grandchild_pid if grandchild_pid > 0 => {
                        let mut status = 0;
                        waitpid(grandchild_pid, &mut status, 0);
                        exit(0);
                    }
                    _ => {
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
        let ret = chdir(cstr!("./merged").as_ptr());
        if ret < 0 {
            bail!("chdir failed")
        };
        let ret = chroot(cstr!("./").as_ptr());
        if ret < 0 {
            bail!("chroot failed")
        };
        Ok(())
    }
}
