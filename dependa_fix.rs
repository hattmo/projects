#!/usr/bin/env -S cargo -Zscript

//! ```cargo
//! [package]
//! edition = '2021'

use std::error::Error;
use std::io::Write;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let out = Command::new("git").args(&["fetch", "--all"]).output()?;
    std::io::stdout().write(&out.stdout)?;
    Ok(())
}
