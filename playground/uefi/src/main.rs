#![no_main]
#![no_std]

use log::info;
use uefi::{
    prelude::*,
    proto::media::file::{File, FileAttribute, FileMode},
    CStr16,
};
static HELLO: &str = "hello";

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    let mut string_buf = [0u16; 100];
    uefi_services::init(&mut system_table).unwrap();
    system_table.boot_services().stall(10_000_000);
    let mut file_sys = system_table
        .boot_services()
        .get_image_file_system(image_handle)
        .unwrap();
    let c16_hello = CStr16::from_str_with_buf(HELLO, &mut string_buf[0..HELLO.len()]).unwrap();
    let mut file = file_sys
        .open_volume()
        .unwrap()
        .open(c16_hello, FileMode::CreateReadWrite, FileAttribute::empty())
        .unwrap().into_regular_file().unwrap();
    file.write("hello world".as_bytes()).unwrap();
    Status::SUCCESS
}
