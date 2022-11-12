use std::{
    error::Error,
    fmt::Display,
    io::{BufReader, Bytes, Read},
};

use self::access_flags::{AccessFlags, ACC_PUBLIC};

pub struct Class {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<Constant>,
    access_flags: AccessFlags,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<Interface>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    attributes: Vec<Attribute>,
}

impl Class {
    pub fn from_reader(mut data: impl Read) -> Result<Class, Box<dyn Error>> {
        let mut reader = BufReader::new(data).bytes();
        // let mut data = data.bytes();
        let magic = reader.next_u32();
        let minor_version = reader.next_u16();
        let major_version = reader.next_u16();
        let access_flags = AccessFlags(ACC_PUBLIC);
        // .next_chunk::<4>().unwrap().map(|i| i.unwrap());

        // let mut buffer = Vec::new();
        // buffer.resize(4, 0);
        // data.read_exact(buffer.as_mut())?;
        // let magic: u32 = u32::from_le_bytes(buffer.as_slice().try_into()?);

        // buffer.resize(2, 0);
        // data.read_exact(buffer.as_mut())?;
        // let minor_version: u16 = u16::from_le_bytes(buffer.as_slice().try_into()?);

        unimplemented!()
    }
}
mod access_flags {
    pub struct AccessFlags(pub u16);
    pub const ACC_PUBLIC: u16 = 0x0001u16;
}
trait Scanner {
    fn next_u32(&mut self) -> Result<u32, ()>;
    fn next_u16(&mut self) -> Result<u16, ()>;
}

impl<T> Scanner for Bytes<T>
where
    T: Read,
{
    fn next_u32(&mut self) -> Result<u32, ()> {
        Ok(u32::from_le_bytes(
            self.next_chunk::<4>()
                .or(Err(()))?
                .try_map(|i| i)
                .or(Err(()))?,
        ))
    }
    fn next_u16(&mut self) -> Result<u16, ()> {
        Ok(u16::from_le_bytes(
            self.next_chunk::<2>()
                .or(Err(()))?
                .try_map(|i| i)
                .or(Err(()))?,
        ))
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hello")
    }
}
struct Attribute;
struct Method;
struct Field;
struct Constant;
struct Interface;
