#![no_main]
#![no_std]

use log::{info, set_logger};
use uefi::logger::Logger;
use uefi::prelude::*;
use uefi_services::{print, println};

static LOGGER: Logger = Logger::new();

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    println!("Hello world");
    let stdout = system_table.stdout() as *mut _;
    unsafe { LOGGER.set_output(stdout) };
    //    set_logger(&LOGGER).inspect_err(|e|{
    //        println!()int)
    //    });

    // info!("hello world");

    let boot_svr = system_table.boot_services();
    boot_svr.stall(1_000_000_000);
    Status::SUCCESS
}
