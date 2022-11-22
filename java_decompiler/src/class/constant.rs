use crate::{ParseFailure, PatchType, Scanner};

use std::{
    borrow::BorrowMut,
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
    constants: Vec<Constant>,
}

impl ConstPool {
    pub fn new() -> Self {
        ConstPool {
            constants: Vec::new(),
        }
    }
    pub fn from_reader(mut reader: Bytes<impl Read>) -> Result<ConstPool, ParseFailure> {
        let mut out = ConstPool::new();
        let const_count = reader.next_u16()?;
        out.constants.push(Constant::Null);
        for _ in 1..const_count {
            let tag = reader.next_u8()?;
            match tag {
                tag::Class => {
                    out.constants.push(Constant::Class {
                        name_index: reader.next_u16()?,
                    });
                }
                tag::Fieldref => {
                    let class_index = reader.next_u16()?;
                    let name_and_type_index = reader.next_u16()?;
                    out.constants.push(Constant::Fieldref {
                        class_index,
                        name_and_type_index,
                    });
                }
                tag::Methodref => {
                    let class_index = reader.next_u16()?;
                    let name_and_type_index = reader.next_u16()?;
                    out.constants.push(Constant::Methodref {
                        class_index,
                        name_and_type_index,
                    });
                }
                tag::InterfaceMethodref => {
                    let class_index = reader.next_u16()?;
                    let name_and_type_index = reader.next_u16()?;
                    out.constants.push(Constant::InterfaceMethodref {
                        class_index,
                        name_and_type_index,
                    });
                }
                tag::String => {
                    out.constants.push(Constant::String {
                        string_index: reader.next_u16()?,
                    });
                }
                tag::Integer => {
                    out.constants.push(Constant::Integer {
                        bytes: reader.next_u32()?,
                    });
                }
                tag::Float => {
                    out.constants.push(Constant::Float {
                        bytes: reader.next_u32()?,
                    });
                }
                tag::Long => {
                    let high_bytes = reader.next_u32()?;
                    let low_bytes = reader.next_u32()?;
                    out.constants.push(Constant::Long {
                        high_bytes,
                        low_bytes,
                    });
                    out.constants.push(Constant::Null)
                }
                tag::Double => {
                    let high_bytes = reader.next_u32()?;
                    let low_bytes = reader.next_u32()?;
                    out.constants.push(Constant::Double {
                        high_bytes,
                        low_bytes,
                    });
                    out.constants.push(Constant::Null)
                }
                tag::NameAndType => {
                    let name_index = reader.next_u16()?;
                    let descriptor_index = reader.next_u16()?;
                    out.constants.push(Constant::NameAndType {
                        name_index,
                        descriptor_index,
                    });
                }
                tag::Utf8 => {}
                tag::MethodHandle => {}
                tag::MethodType => {}
                tag::InvokeDynamic => {}
                _ => return Err(ParseFailure),
            }
        }
        Ok(out)
    }

    pub fn insert_const(&mut self, index: u16, item: Constant) -> Result<(), ()> {
        let index_usize: usize = index.try_into().or(Err(()))?;
        if let Constant::Null = self.constants.get(index_usize).ok_or(())? {
            return Err(());
        }
        self.constants.insert(index_usize, item);
        Ok(())
    }

    fn patch_pointers(
        &mut self,
        modified_index: u16,
        transform: fn(&mut &mut u16) -> (),
        patch_type: PatchType,
    ) -> Result<(), ()> {
        for constant in self.constants.iter_mut() {
            if constant.get_pointers().iter().any(|i| **i == modified_index) {};
        }

        for constant in self.constants.iter_mut() {
            constant
                .get_pointers()
                .iter_mut()
                .filter(|i| ***i >= modified_index)
                .for_each(transform);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{ConstPool, Constant};

    #[test]
    fn foo() {
        let mut foo = ConstPool::new();
        foo.constants.push(Constant::Null);
        foo.constants.push(Constant::Class { name_index: 2 });
        foo.patch_pointers(0, |i| **i -= 1, crate::PatchType::Remove);
        println!("{:?}", foo.constants);
    }
}
