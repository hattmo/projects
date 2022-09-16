#![no_std]
#![feature(pointer_byte_offsets)]
use core::alloc::{GlobalAlloc, Layout};

struct MyAllocator;

impl MyAllocator {
    fn new() -> Self {
        Self
    }
}

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

pub mod prelude {}
