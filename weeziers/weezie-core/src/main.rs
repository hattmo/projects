#![no_main]
#![no_std]
#![feature(lang_items)]

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn start() -> ! {
    loop {}
}
