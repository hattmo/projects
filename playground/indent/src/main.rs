#![feature(slice_pattern)]
use core::slice::SlicePattern;
use std::{fs, io, os::unix::fs::FileTypeExt, path::Path};
fn main() {
    if let [_, path] = std::env::args().collect::<Box<[_]>>().as_slice() {
        let _ = do_thing(path);
    } else {
        eprintln!("Please provide a path");
    };
}
fn do_thing(input: impl AsRef<Path>) -> io::Result<()> {
    let paths = fs::read_dir(input);
    let paths = paths.inspect_err(|e| {
        eprintln!("Error: {e}");
    })?;

    paths
        .into_iter()
        .filter_map(|path| {
            path.inspect_err(|e| {
                eprintln!("Error: {e}");
            })
            .ok()
        })
        .for_each(|path| {
            let name = path.file_name();
            let name = name.to_string_lossy();
            match path.file_type() {
                Ok(ty) if ty.is_dir() => {
                    println!("{name} is a dir");
                }
                Ok(ty) if ty.is_fifo() => {
                    println!("{name} is a fifo");
                }
                Ok(ty) if ty.is_file() => {
                    println!("{name} is a file");
                }
                Ok(ty) if ty.is_socket() => {
                    println!("{name} is a socket");
                }
                Ok(ty) if ty.is_symlink() => {
                    println!("{name} is a symlink");
                }
                Ok(ty) if ty.is_char_device() => {
                    println!("{name} is a char device");
                }
                Ok(ty) if ty.is_block_device() => {
                    println!("{name} is a block device");
                }
                Ok(_) => {
                    println!("{name} is a something else");
                }
                Err(e) => {
                    eprintln!("Error: {e}")
                }
            }
        });
    Ok(())
}
