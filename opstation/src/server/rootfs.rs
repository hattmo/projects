use libc::{umount, MS_RDONLY};
use std::{
    ffi::CString,
    fs::{self},
    io,
    path::{Path, PathBuf},
};

use crate::{
    raw::{mount, LoopDev},
    util::AsCString,
};

pub struct RootFS {
    root_path: PathBuf,
    loop_dev: LoopDev,
}

impl RootFS {
    pub fn new(bin_path: impl AsRef<Path>) -> io::Result<RootFS> {
        let bin_path: &Path = bin_path.as_ref();
        let id = uuid::Uuid::new_v4();
        let root_path: PathBuf = format!("/tmp/opstation/{}", id).into();
        for folder in ["root", "changes", "work", "src"] {
            fs::create_dir_all(root_path.join(folder))?;
        }
        let loop_dev = LoopDev::new()?;
        loop_dev.attach_file(bin_path)?;
        mount(
            Some(&loop_dev.path().as_cstring()?),
            &root_path.join("src").as_cstring()?,
            Some(c"ext4"),
            MS_RDONLY,
            None,
        )?;
        mount(
            Some(c"overlay"),
            &root_path.join("root").as_cstring()?,
            Some(c"overlay"),
            0,
            Some(&CString::new(format!(
                "lowerdir={},upperdir={},workdir={}",
                root_path.join("src").display(),
                root_path.join("changes").display(),
                root_path.join("work").display()
            ))?),
        )?;
        Ok(RootFS {
            root_path,
            loop_dev,
        })
    }

    pub fn remove(self) -> io::Result<()> {
        unsafe { umount(self.root_path.join("root").as_cstring()?.as_ptr()) };
        unsafe { umount(self.root_path.join("image").as_cstring()?.as_ptr()) };
        fs::remove_dir_all(&self.root_path);
        Ok(())
    }
}
