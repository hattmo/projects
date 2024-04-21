#![no_main]
#![no_std]

extern crate alloc;

use log::{error, info};
use uefi::prelude::*;

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();
    let vendor = system_table.firmware_vendor();
    let services = system_table.boot_services();
    let mut buffer = [0u8; 10000];
    let map = match services.memory_map(&mut buffer) {
        Ok(map) => map,
        Err(err) => {
            error!("error: {err}");
            loop {}
        }
    };
    for item in map.entries() {
        info!("{item:?}");
    }
    info!("{vendor}");
    loop {}
}
