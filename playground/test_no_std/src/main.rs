#![feature(alloc_error_handler)]
#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]

use core::{alloc::GlobalAlloc, arch::asm, panic::PanicInfo, ptr, sync::atomic::AtomicPtr};
extern crate alloc;
use alloc::vec::Vec;

struct MyAllocator {
    blob: AtomicPtr<u8>,
}

impl MyAllocator {
    fn initialize(&self) {}
}

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        ALLOCATOR.blob = AtomicPtr::new(ptr::null_mut());
        mmap(500)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {}
}

#[global_allocator]
static mut ALLOCATOR: MyAllocator = MyAllocator {
    blob: AtomicPtr::new(ptr::null_mut()),
};

#[alloc_error_handler]
fn foo(layout: core::alloc::Layout) -> ! {
    exit(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut bar: Vec<u8> = Vec::with_capacity(10);
    bar.push(30);
    bar.push(20);

    exit(bar.last().unwrap().clone().into())
}

#[no_mangle]
pub extern "C" fn memcpy(dst: *mut u8, src: *const u8, size: isize) {
    unsafe {
        let mut cursor = ptr::null_mut();
        for off in 0..size {
            cursor = dst.byte_offset(off);
            *cursor = *src.byte_offset(off);
        }
    }
}

fn exit(exit_code: i64) -> ! {
    unsafe {
        asm!(
            "mov rdi, {0:r}",
            "mov rax, 0x3c",
            "syscall",
            in(reg) exit_code,
            options(nostack, noreturn)
        )
    }
}

fn mmap(size: usize) -> *mut u8 {
    unsafe {
        let mut out: i64 = 0;
        asm! {
            "mov rax, 9",
            "mov rdi, 0",
            "mov rdx, 3",
            "mov r10, 34",
            "mov r8, -1",
            "mov r9, 0",
            in("rsi") size,
            out("rax") out,
        }
        out as *mut u8
    }
}
// RUSTFLAGS="-Ctarget-cpu=native -Clink-args=-nostartfiles -Clink-args=-Wl,-n,-N,--no-dynamic-linker,--build-id=none" cargo build --release