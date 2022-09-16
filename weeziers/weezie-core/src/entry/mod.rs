use core::arch::asm;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    crate::main();
    unsafe {
        asm! {
            "mov rax, 0x3c",
            "mov rdi, 0",
            "syscall",
            options(noreturn)
        }
    }
}
