use anyhow::Result;
use std::path::Path;

use crate::machine::Machine;

pub fn server_main() -> Result<()> {
    let _machine = Machine::new(Path::new("./arch.img"))?;
    Ok(())
}
