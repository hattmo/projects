use anyhow::{bail, Result};
use libc::ioctl;
use nix::mount::{mount, umount, MsFlags};
use std::{
    fs::{self, OpenOptions},
    io::Error,
    os::fd::AsRawFd,
    path::{Path, PathBuf},
    process::Child,
};

#[derive(Debug)]
pub struct Machine {
    machine_path: PathBuf,
    loop_path: PathBuf,
    manager: Child,
}

impl Machine {
    pub fn new(image_path: &Path) -> Result<Machine> {
        let id = uuid::Uuid::new_v4();
        let machine_path: PathBuf = format!("/tmp/opstation/{}", id).into();
        for folder in ["root", "changes", "work", "image"] {
            fs::create_dir_all(machine_path.join(folder))?;
        }
        let lc = OpenOptions::new()
            .write(true)
            .create(true)
            .open("/dev/loop-control")?;
        let open_ld = unsafe { ioctl(lc.as_raw_fd(), 0x4C82) };
        let loop_path: PathBuf = format!("/dev/loop{}", open_ld).into();
        let loop_device = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&loop_path)?;
        let image = OpenOptions::new().write(true).read(true).open(image_path)?;
        let set_fd_result = unsafe { ioctl(loop_device.as_raw_fd(), 0x4C00, image.as_raw_fd()) };
        if set_fd_result != 0 {
            bail!("failed to set fd")
        };
        mount(
            Some(&loop_path),
            &machine_path.join("image"),
            Some("ext4"),
            MsFlags::MS_RDONLY,
            Option::<&str>::None,
        )?;
        mount(
            Some("overlay"),
            &machine_path.join("root"),
            Some("overlay"),
            MsFlags::empty(),
            Some(
                format!(
                    "lowerdir={},upperdir={},workdir={}",
                    machine_path.join("image").display(),
                    machine_path.join("changes").display(),
                    machine_path.join("work").display()
                )
                .as_str(),
            ),
        )?;
        let mut manager = std::process::Command::new(std::env::current_exe()?)
            .arg("--manager")
            .current_dir(machine_path.join("root"))
            .spawn()?;
        manager.wait()?;
        Ok(Machine {
            machine_path,
            loop_path,
            manager,
        })
    }
}

impl Drop for Machine {
    fn drop(&mut self) {
        let results = [
            umount(&self.machine_path.join("root")).map_err(Error::other),
            umount(&self.machine_path.join("image")).map_err(Error::other),
            self.manager.kill(),
            self.manager.wait().map(|_| ()),
            fs::remove_dir_all(&self.machine_path),
        ];
        if let Ok(loop_device) = OpenOptions::new()
            .write(true)
            .read(true)
            .open(&self.loop_path)
        {
            let res = unsafe { ioctl(loop_device.as_raw_fd(), 0x4C01) };
            if res != 0 {
                log::error!("Failed to release loop device");
            }
        } else {
            log::error!("Failed to open loop device");
        }
        results.into_iter().filter_map(|a| a.err()).for_each(|e| {
            log::error!("{:?}", e);
        });
    }
}
