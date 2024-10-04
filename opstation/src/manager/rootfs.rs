use libc::MS_RDONLY;
use std::{
    ffi::CString,
    fs::{self},
    io,
    path::{Path, PathBuf},
};

use crate::{
    raw::{mount, umount, LoopDev},
    util::AsCString,
};

pub struct RootFS {
    base_path: PathBuf,
    root_path: PathBuf,
    loop_dev: LoopDev,
}

impl RootFS {
    pub fn new(bin_path: impl AsRef<Path>) -> io::Result<RootFS> {
        let bin_path: &Path = bin_path.as_ref();
        let id = uuid::Uuid::new_v4();
        let base_path: PathBuf = format!("/tmp/opstation/{}", id).into();
        for folder in ["root", "changes", "work", "src"] {
            fs::create_dir_all(base_path.join(folder))?;
        }
        let loop_dev = LoopDev::new()?;
        loop_dev.attach_file(bin_path)?;
        mount(
            Some(&loop_dev.path().as_cstring()),
            &base_path.join("src").as_cstring(),
            Some(c"ext4"),
            MS_RDONLY,
            None,
        )?;
        mount(
            Some(c"overlay"),
            &base_path.join("root").as_cstring(),
            Some(c"overlay"),
            0,
            Some(&CString::new(format!(
                "lowerdir={},upperdir={},workdir={}",
                base_path.join("src").display(),
                base_path.join("changes").display(),
                base_path.join("work").display()
            ))?),
        )?;
        Ok(RootFS {
            base_path: base_path.clone(),
            root_path: base_path.join("root"),
            loop_dev,
        })
    }

    fn remove(&self) -> io::Result<()> {
        umount(&self.base_path.join("root").as_cstring())?;
        umount(&self.base_path.join("src").as_cstring())?;
        fs::remove_dir_all(&self.base_path)?;
        self.loop_dev.detach_file()?;
        Ok(())
    }
    pub fn path(&self) -> &Path {
        &self.root_path
    }
}

impl Drop for RootFS {
    fn drop(&mut self) {
        if let Err(err) = self.remove() {
            log::error!("Error cleaning RootFS: {err})");
        };
    }
}
