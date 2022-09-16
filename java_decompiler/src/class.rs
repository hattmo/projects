use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    error::{Error, ErrorKind},
    number::complete::be_u16,
    sequence::{preceded, tuple},
    Err, IResult,
};

use crate::constant::{const_types::Class_info, ConstPool, ConstValue, ConstantType};

use bitflags::bitflags;

bitflags! {
    pub struct AccessFlags: u16 {
        const ACC_PUBLIC = 0x0001;
        const ACC_FINAL = 0x0010;
        const ACC_SUPER = 0x0020;
        const ACC_INTERFACE = 0x0200;
        const ACC_ABSTRACT = 0x0400;
        const ACC_SYNTHETIC = 0x1000;
        const ACC_ANNOTATION = 0x2000;
        const ACC_ENUM = 0x4000;
    }
}

impl Display for AccessFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out: String = self.iter_names().map(|(s, _)| s).intersperse("|").collect();
        write!(f, "{out}")
    }
}

pub struct Class {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstPool,
    pub access_flags: AccessFlags,
    pub this_class: ConstValue<Class_info>,
    pub super_class: ConstValue<Class_info>,
    // interfaces: Vec<Interface>,
    // fields: Vec<Field>,
    // methods: Vec<Method>,
    // attributes: Vec<Attribute>,
}

impl Class {
    pub fn new() -> Self {
        let mut constant_pool = ConstPool::new();
        let this_class_name = constant_pool.add_utf8("NewClass");
        let this_class = constant_pool.add_class(&this_class_name);
        let super_class_name = constant_pool.add_utf8("java/lang/Object");
        let super_class = constant_pool.add_class(&super_class_name);
        Class {
            minor_version: 0,
            major_version: 61,
            constant_pool,
            access_flags: AccessFlags::ACC_PUBLIC | AccessFlags::ACC_SUPER,
            this_class,
            super_class,
        }
    }
    pub fn from_slice(data: &[u8]) -> IResult<&[u8], Class> {
        let (data, (minor_version, major_version)) =
            preceded(tag(b"\xCA\xFE\xBA\xBE"), tuple((be_u16, be_u16)))(data)?;
        let (data, constant_pool) = ConstPool::from_slice(data)?;
        let (data, (access_flags, this_class, super_class)) =
            tuple((be_u16, be_u16, be_u16))(data)?;
        if let (
            Some(ConstantType::Class_info(this_class)),
            Some(ConstantType::Class_info(super_class)),
        ) = (
            constant_pool.get_const(this_class),
            constant_pool.get_const(super_class),
        ) {
            let access_flags = AccessFlags::from_bits_truncate(access_flags);
            Ok((
                data,
                Class {
                    access_flags,
                    major_version,
                    minor_version,
                    super_class,
                    this_class,
                    constant_pool,
                },
            ))
        } else {
            Err(Err::Error(Error::new(data, ErrorKind::Verify)))
        }
    }
}

impl Default for Class {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "minor_version: {}\n", self.minor_version)?;
        writeln!(f, "major_version: {}\n", self.major_version)?;
        writeln!(f, "constant_pool: {}\n", self.constant_pool)?;
        writeln!(f, "access_flags: {}\n", self.access_flags)?;
        writeln!(
            f,
            "this_class: {}\n",
            self.this_class.get_name_index().get_string()
        )?;
        writeln!(
            f,
            "super_class: {}\n",
            self.super_class.get_name_index().get_string()
        )?;
        Ok(())
    }
}
