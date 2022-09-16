use std::{error::Error, fmt::Display, io::Read};

use crate::Scanner;
mod constant;
use constant::ConstPool;
#[derive(Debug)]
pub struct Class {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    // constant_pool: ConstPool,
    // access_flags: unimplemented!(),
    this_class: u16,
    super_class: u16,
    // interfaces: Vec<Interface>,
    // fields: Vec<Field>,
    // methods: Vec<Method>,
    // attributes: Vec<Attribute>,
}

impl Class {
    pub fn from_reader(mut data: impl Read) -> Result<Class, Box<dyn Error>> {
        let mut reader = data.bytes();
        // let mut data = data.bytes();
        let magic = reader.next_u32()?;
        let minor_version = reader.next_u16()?;
        let major_version = reader.next_u16()?;
        // let constant_pool = ConstPool::from_reader(reader)?;

        // Ok(Class {
        //     magic,
        //     minor_version,
        //     major_version,
        //     constant_pool,
        //     access_flags: (),
        //     this_class: (),
        //     super_class: (),
        //     interfaces: (),
        //     fields: (),
        //     methods: (),
        //     attributes: (),
        // })
        Err("foo".into())
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hello")
    }
}
