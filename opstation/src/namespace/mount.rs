use std::ptr;

use cstr::cstr;
use libc::{
    c_void, makedev, mkdir, mknod, mount, symlink, MS_NODEV, MS_NOEXEC, MS_NOSUID, MS_RDONLY,
    MS_STRICTATIME, S_IFCHR,
};
pub fn setup_mounts() {
    unsafe {
        mkdir(cstr!("/proc").as_ptr(), 0o755);
        mount(
            cstr!("proc").as_ptr(),
            cstr!("/proc").as_ptr(),
            cstr!("proc").as_ptr(),
            MS_NOEXEC | MS_NOSUID | MS_NODEV,
            ptr::null(),
        );
        mkdir(cstr!("/dev").as_ptr(), 0o755);
        mount(
            cstr!("tmpfs").as_ptr(),
            cstr!("/dev").as_ptr(),
            cstr!("tmpfs").as_ptr(),
            MS_NOEXEC | MS_STRICTATIME,
            cstr!("mode=755").as_ptr() as *const c_void,
        );
        mkdir(cstr!("/dev/shm").as_ptr(), 0o755);
        mount(
            cstr!("shm").as_ptr(),
            cstr!("/dev/shm").as_ptr(),
            cstr!("tmpfs").as_ptr(),
            MS_NOEXEC | MS_NOSUID | MS_NODEV,
            cstr!("mode=1777,size=65536k").as_ptr() as *const c_void,
        );

        mkdir(cstr!("/dev/mqueue").as_ptr(), 0o755);
        mount(
            cstr!("mqueue").as_ptr(),
            cstr!("/dev/mqueue").as_ptr(),
            cstr!("mqueue").as_ptr(),
            MS_NOEXEC | MS_NOSUID | MS_NODEV,
            ptr::null(),
        );

        mkdir(cstr!("/dev/pts").as_ptr(), 0o755);
        mount(
            cstr!("devpts").as_ptr(),
            cstr!("/dev/pts").as_ptr(),
            cstr!("devpts").as_ptr(),
            MS_NOEXEC | MS_NOSUID,
            cstr!("newinstance,ptmxmode=0666,mode=620,gid=5").as_ptr() as *const c_void,
        );
        mkdir(cstr!("/sys").as_ptr(), 0o755);
        mount(
            cstr!("sysfs").as_ptr(),
            cstr!("/sys").as_ptr(),
            cstr!("sysfs").as_ptr(),
            MS_NOEXEC | MS_NOSUID | MS_NODEV | MS_RDONLY,
            ptr::null(),
        );
    }
}

pub fn setup_devices() {
    unsafe {
        const MODE: u32 = 0o666 | S_IFCHR;
        mknod(cstr!("/dev/null").as_ptr(), MODE, makedev(1, 3));
        mknod(cstr!("/dev/zero").as_ptr(), MODE, makedev(1, 5));
        mknod(cstr!("/dev/full").as_ptr(), MODE, makedev(1, 7));
        mknod(cstr!("/dev/tty").as_ptr(), MODE, makedev(5, 0));
        mknod(cstr!("/dev/random").as_ptr(), MODE, makedev(1, 8));
        mknod(cstr!("/dev/urandom").as_ptr(), MODE, makedev(1, 9));
    }
}

pub fn setup_io_link() {
    unsafe {
        symlink(cstr!("/proc/self/fd").as_ptr(), cstr!("/dev/fd").as_ptr());
        symlink(
            cstr!("/proc/self/fd/0").as_ptr(),
            cstr!("/dev/stdin").as_ptr(),
        );
        symlink(
            cstr!("/proc/self/fd/1").as_ptr(),
            cstr!("/dev/stdout").as_ptr(),
        );
        symlink(
            cstr!("/proc/self/fd/2").as_ptr(),
            cstr!("/dev/stderr").as_ptr(),
        );
    }
}
