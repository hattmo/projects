#![no_std]

use core::{
    arch::asm,
    future::Future,
    pin::pin,
    ptr,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

mod net;

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {}
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

static V_TABLE: RawWakerVTable = {
    unsafe fn clone(data: *const ()) -> RawWaker {
        RawWaker::new(data, &V_TABLE)
        // ...
    }
    unsafe fn wake(_data: *const ()) {
        // ...
    }
    unsafe fn wake_by_ref(_data: *const ()) {
        // ...
    }
    unsafe fn drop(_data: *const ()) {
        asm! {
            "mov rdi, rsi",
        }
        // ...
    }
    RawWakerVTable::new(clone, wake, wake_by_ref, drop)
};

impl Runtime {
    pub fn block_on<F, R, T>(self, entry: R) -> T
    where
        R: FnOnce(Runtime) -> F,
        F: Future<Output = T>,
    {
        let mut fut = pin!(entry(self));
        let waker = RawWaker::new(ptr::null(), &V_TABLE);
        let waker = unsafe { Waker::from_raw(waker) };
        let mut cx = Context::from_waker(&waker);
        loop {
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(val) => return val,
                Poll::Pending => {}
            }
        }
    }
}
