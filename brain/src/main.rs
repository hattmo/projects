#![feature(iter_intersperse)]

use brain::SharedMemory;
use colored::Colorize;
use std::num::NonZeroUsize;
fn main() {
    let (dma, path) = SharedMemory::new(Some("foo")).unwrap();
    println!("Path: {path:?}");
    let foo: &[u8] = dma
        .get_slice(0x0, NonZeroUsize::new(0x10).expect("not zero"))
        .unwrap();
    loop {
        println!("{}", foo.to_bytes_string());
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

// fn print_bytes(bytes: &[u8]) {
//     let out: String = bytes
//         .iter()
//         .map(|i| {
//             if i.is_ascii_alphanumeric() {
//                 format!("{}", *i as char)
//             } else if *i == 0 {
//                 "_".to_string()
//             } else {
//                 format!("0x{i:02.2x}")
//             }
//         })
//         .intersperse(", ".to_string())
//         .collect();
//     println!("[{}]", out);
// }

trait ByteString {
    fn to_bytes_string(&self) -> String;
}

impl ByteString for &[u8] {
    fn to_bytes_string(&self) -> String {
        impl_to_bytes_string(self)
    }
}
impl ByteString for &mut [u8] {
    fn to_bytes_string(&self) -> String {
        impl_to_bytes_string(self)
    }
}

fn impl_to_bytes_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|i| {
            if i.is_ascii_alphanumeric() {
                format!("{}", *i as char).green()
            } else if *i == 0 {
                "_".to_string().normal()
            } else {
                format!("0x{i:02.2x}").red()
            }
        })
        .map(|s| s.to_string())
        .collect()
}
