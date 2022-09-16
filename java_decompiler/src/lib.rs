#![feature(iter_next_chunk)]
#![feature(array_try_map)]
#![feature(iterator_try_collect)]
#![feature(provide_any)]
mod class;
use std::io::{Bytes, Read};

// pub use class::Class;
use thiserror::Error;
#[derive(Error, Debug)]
#[error("Failed to parse")]
pub struct ParseFailure;
trait Scanner {
    fn next_u32(&mut self) -> Result<u32, ParseFailure>;
    fn next_u16(&mut self) -> Result<u16, ParseFailure>;
    fn next_u8(&mut self) -> Result<u8, ParseFailure>;
}

impl<T> Scanner for Bytes<T>
where
    T: Read,
{
    fn next_u32(&mut self) -> Result<u32, ParseFailure> {
        Ok(u32::from_be_bytes(
            self.next_chunk::<4>()
                .or(Err(ParseFailure))?
                .try_map(|i| i)
                .or(Err(ParseFailure))?,
        ))
    }
    fn next_u16(&mut self) -> Result<u16, ParseFailure> {
        Ok(u16::from_be_bytes(
            self.next_chunk::<2>()
                .or(Err(ParseFailure))?
                .try_map(|i| i)
                .or(Err(ParseFailure))?,
        ))
    }
    fn next_u8(&mut self) -> Result<u8, ParseFailure> {
        Ok(self.next().ok_or(ParseFailure)?.or(Err(ParseFailure))?)
    }
}

enum PatchType {
    Insert,
    Remove,
}
