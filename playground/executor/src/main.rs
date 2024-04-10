use std::{
    cell::Cell,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use executor::Runtime;

fn main() {
    let runtime = Runtime::new();
    runtime.block_on(start)
}

async fn start(run: Runtime) {}
// fn print(text: &str) {
//     unsafe {
//         asm! {
//             "mov rax, 1",
//             "mov rdi, 1",
//             "mov rdi, rsi",
//             "syscall",
//             in("rsi") text.as_ptr(),
//             in("rdx") text.len(),
//             out("rax") _,
//         }
//     }
// }
