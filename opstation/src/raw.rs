use std::{ffi::CString, path::Path};

use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use libc::{c_ulong, c_void};
use std::sync::LazyLock;
use uuid::Uuid;
/*pub fn mount(
    src: &str,
    target: &str,
    fstype: &str,
    flags: c_ulong,
    data: Option<&str>,
) -> Result<()> {
    let src = CString::new(src)?;
    let target = CString::new(target)?;
    let fstype = CString::new(fstype)?;
    let ret = unsafe {
        libc::mount(
            src.as_ptr(),
            target.as_ptr(),
            fstype.as_ptr(),
            flags,
            data.unwrap_or(std::ptr::null()),
        )
    };
    Ok(())
}
*/

static DIRS: LazyLock<Option<ProjectDirs>> =
    LazyLock::new(|| ProjectDirs::from("com", "hattmo", "opstation"));

pub fn mount_layers(layers: &[&str]) -> Result<PathBuf> {
    let runtime_dir = DIRS
        .and_then(|dirs| dirs.runtime_dir())
        .ok_or(anyhow!("Failed to "))?;
    let to = CString::new(to.display().to_string()).unwrap();
    let ret = unsafe {
        libc::mount(
            c"overlay".as_ptr(),
            to.as_ptr(),
            c"overlay".as_ptr(),
            0,
            CString::new(format!(
                "lowerdir={},upperdir={},workdir={}",
                from.display(),
                machine_path.join("changes").display(),
                machine_path.join("work").display()
            ))
            .as_ptr(),
        )
    };
    Ok(())
}
