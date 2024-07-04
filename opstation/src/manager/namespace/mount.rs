use std::ptr;

use libc::{
    c_void, makedev, mkdir, mknod, mount, symlink, MS_NODEV, MS_NOEXEC, MS_NOSUID, MS_RDONLY,
    MS_STRICTATIME, S_IFCHR,
};
pub fn setup_mounts() {
    unsafe {
        mkdir(c"/proc".as_ptr(), 0o755);
        mount(
            c"proc".as_ptr(),
            c"/proc".as_ptr(),
            c"proc".as_ptr(),
            MS_NOEXEC | MS_NOSUID | MS_NODEV,
            ptr::null(),
        );
        mkdir(c"/dev".as_ptr(), 0o755);
        mount(
            c"tmpfs".as_ptr(),
            c"/dev".as_ptr(),
            c"tmpfs".as_ptr(),
            MS_NOEXEC | MS_STRICTATIME,
            c"mode=755".as_ptr() as *const c_void,
        );
        mkdir(c"/dev/shm".as_ptr(), 0o755);
        mount(
            c"shm".as_ptr(),
            c"/dev/shm".as_ptr(),
            c"tmpfs".as_ptr(),
            MS_NOEXEC | MS_NOSUID | MS_NODEV,
            c"mode=1777,size=65536k".as_ptr() as *const c_void,
        );

        mkdir(c"/dev/mqueue".as_ptr(), 0o755);
        mount(
            c"mqueue".as_ptr(),
            c"/dev/mqueue".as_ptr(),
            c"mqueue".as_ptr(),
            MS_NOEXEC | MS_NOSUID | MS_NODEV,
            ptr::null(),
        );

        mkdir(c"/dev/pts".as_ptr(), 0o755);
        mount(
            c"devpts".as_ptr(),
            c"/dev/pts".as_ptr(),
            c"devpts".as_ptr(),
            MS_NOEXEC | MS_NOSUID,
            c"newinstance,ptmxmode=0666,mode=620,gid=5".as_ptr() as *const c_void,
        );
        mkdir(c"/sys".as_ptr(), 0o755);
        mount(
            c"sysfs".as_ptr(),
            c"/sys".as_ptr(),
            c"sysfs".as_ptr(),
            MS_NOEXEC | MS_NOSUID | MS_NODEV | MS_RDONLY,
            ptr::null(),
        );
    }
}

pub fn setup_devices() {
    unsafe {
        const MODE: u32 = 0o666 | S_IFCHR;
        mknod(c"/dev/null".as_ptr(), MODE, makedev(1, 3));
        mknod(c"/dev/zero".as_ptr(), MODE, makedev(1, 5));
        mknod(c"/dev/full".as_ptr(), MODE, makedev(1, 7));
        mknod(c"/dev/tty".as_ptr(), MODE, makedev(5, 0));
        mknod(c"/dev/random".as_ptr(), MODE, makedev(1, 8));
        mknod(c"/dev/urandom".as_ptr(), MODE, makedev(1, 9));
    }
}

pub fn setup_io_link() {
    unsafe {
        symlink(c"/proc/self/fd".as_ptr(), c"/dev/fd".as_ptr());
        symlink(c"/proc/self/fd/0".as_ptr(), c"/dev/stdin".as_ptr());
        symlink(c"/proc/self/fd/1".as_ptr(), c"/dev/stdout".as_ptr());
        symlink(c"/proc/self/fd/2".as_ptr(), c"/dev/stderr".as_ptr());
    }
}
