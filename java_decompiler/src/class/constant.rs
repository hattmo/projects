use std::{
    collections::HashMap,
    hash,
    io::{Bytes, Read},
};

mod tag {
    pub const Class: u8 = 7;
    pub const Fieldref: u8 = 9;
    pub const Methodref: u8 = 10;
    pub const InterfaceMethodref: u8 = 11;
    pub const String: u8 = 8;
    pub const Integer: u8 = 3;
    pub const Float: u8 = 4;
    pub const Long: u8 = 5;
    pub const Double: u8 = 6;
    pub const NameAndType: u8 = 12;
    pub const Utf8: u8 = 1;
    pub const MethodHandle: u8 = 15;
    pub const MethodType: u8 = 16;
    pub const InvokeDynamic: u8 = 18;
}

#[derive(Debug)]
enum Constant {
    Null,
    Class {
        name_index: u16,
    },
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    String {
        string_index: u16,
    },
    Integer {
        bytes: u32,
    },
    Float {
        bytes: u32,
    },
    Long {
        high_bytes: u32,
        low_bytes: u32,
    },
    Double {
        high_bytes: u32,
        low_bytes: u32,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf8 {
        length: u16,
        bytes: String,
    },
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}

impl Constant {
    fn get_pointers(&mut self) -> Vec<&mut u16> {
        match self {
            Constant::Class { name_index } => {
                return vec![name_index];
            }
            Constant::Fieldref {
                class_index,
                name_and_type_index,
            } => {
                return vec![class_index, name_and_type_index];
            }
            Constant::Methodref {
                class_index,
                name_and_type_index,
            } => {
                return vec![class_index, name_and_type_index];
            }
            Constant::InterfaceMethodref {
                class_index,
                name_and_type_index,
            } => {
                return vec![class_index, name_and_type_index];
            }
            Constant::String { string_index } => {
                return vec![string_index];
            }
            Constant::NameAndType {
                name_index,
                descriptor_index,
            } => {
                return vec![name_index, descriptor_index];
            }
            Constant::MethodHandle {
                reference_kind: _,
                reference_index,
            } => {
                return vec![reference_index];
            }
            Constant::MethodType { descriptor_index } => {
                return vec![descriptor_index];
            }
            Constant::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                return vec![bootstrap_method_attr_index, name_and_type_index];
            }
            _ => return Vec::new(),
        }
    }
}

pub struct ConstPool {
    constants: HashMap<u16, Constant>,
}

impl ConstPool {
    pub fn new() -> Self {
        ConstPool {
            constants: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ConstPool, Constant};

    #[test]
    fn foo() {}
}
