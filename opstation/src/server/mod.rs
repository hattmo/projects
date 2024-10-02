mod rootfs;

use rootfs::RootFS;
use std::{io, path::Path};

pub fn server_main() -> io::Result<()> {
    let root_fs = RootFS::new(Path::new("./arch.img"))?;
    root_fs.remove();
    Ok(())
}
