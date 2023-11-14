#![feature(iter_intersperse)]
use std::fmt::Display;

use anyhow::Result;

trait SliceOffset<T> {
    fn slice_offset(&self, offset: usize, size: usize) -> &[T];
}
impl<T> SliceOffset<T> for Vec<T> {
    fn slice_offset(&self, offset: usize, size: usize) -> &[T] {
        &self[offset..offset + size]
    }
}

struct Bytes<T>(T)
where
    T: AsRef<[u8]>;

impl<T> Display for Bytes<T>
where
    T: AsRef<[u8]>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self
            .0
            .as_ref()
            .iter()
            .map(|c| {
                if c.is_ascii_alphanumeric() {
                    format!("{}", char::from(*c))
                } else {
                    format!("0x{:x?}", c)
                }
            })
            .intersperse(", ".to_owned())
            .collect::<String>();

        write!(f, "[{}]", inner)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let elf = std::fs::read("./src/test")?;
    println!("magic: {}", Bytes(elf.slice_offset(0, 4)));
    let bits64 = elf.slice_offset(0x04, 1)[0] == 2;
    println!("64 bit: {}", bits64);
    let object_type = u16::from_le_bytes(elf.slice_offset(0x10, 2).try_into().unwrap());
    println!("Object type: {}", object_type);
    let entry_point = u64::from_le_bytes(elf.slice_offset(0x18, 8).try_into().unwrap());
    println!("Entry point: 0x{:x}", entry_point);
    Ok(())
}
