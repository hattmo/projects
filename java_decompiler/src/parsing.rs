use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map_res,
    multi::length_data,
    number::complete::{be_u16, be_u32, be_u64, be_u8, double, float},
    sequence::{preceded, tuple},
    IResult,
};

use crate::constant::const_types::*;
use crate::constant::{ConstPool, ConstValue, ConstantType};

impl ConstPool {
    pub fn from_slice(data: &[u8]) -> IResult<&[u8], ConstPool> {
        let mut pool = ConstPool::new();
        let (mut rest, const_count) = be_u16(data)?;
        let mut curr_index = 0;
        while curr_index < (const_count - 1) {
            let (new_rest, const_type) = Self::parse_const_item(rest, &mut pool)?;
            rest = new_rest;
            curr_index = pool.add_raw_const(const_type);
        }
        Ok((rest, pool))
    }
    fn parse_const_item<'data>(
        data: &'data [u8],
        pool: &mut ConstPool,
    ) -> IResult<&'data [u8], ConstantType> {
        alt((
            map_res(preceded(tag([7]), be_u16), |name_index| {
                if let Some(ConstantType::Utf8_info(name_index)) = pool.get_const(name_index) {
                    Ok(ConstantType::Class_info(ConstValue::new(
                        0,
                        Class_info { name_index },
                    )))
                } else {
                    Err("Invalid references")
                }
            }),
            map_res(
                preceded(tag([9]), tuple((be_u16, be_u16))),
                |(class_index, name_and_type_index)| {
                    if let (
                        Some(ConstantType::Class_info(class_index)),
                        Some(ConstantType::NameAndType_info(name_and_type_index)),
                    ) = (
                        pool.get_const(class_index),
                        pool.get_const(name_and_type_index),
                    ) {
                        Ok(ConstantType::Fieldref_info(ConstValue::new(
                            0,
                            Fieldref_info {
                                class_index,
                                name_and_type_index,
                            },
                        )))
                    } else {
                        Err("Invalid references")
                    }
                },
            ),
            map_res(
                preceded(tag([10]), tuple((be_u16, be_u16))),
                |(class_index, name_and_type_index)| {
                    if let (
                        Some(ConstantType::Class_info(class_index)),
                        Some(ConstantType::NameAndType_info(name_and_type_index)),
                    ) = (
                        pool.get_const(class_index),
                        pool.get_const(name_and_type_index),
                    ) {
                        Ok(ConstantType::Methodref_info(ConstValue::new(
                            0,
                            Methodref_info {
                                class_index,
                                name_and_type_index,
                            },
                        )))
                    } else {
                        Err("Invalid references")
                    }
                },
            ),
            map_res(
                preceded(tag([11]), tuple((be_u16, be_u16))),
                |(class_index, name_and_type_index)| {
                    if let (
                        Some(ConstantType::Class_info(class_index)),
                        Some(ConstantType::NameAndType_info(name_and_type_index)),
                    ) = (
                        pool.get_const(class_index),
                        pool.get_const(name_and_type_index),
                    ) {
                        Ok(ConstantType::InterfaceMethodref_info(ConstValue::new(
                            0,
                            InterfaceMethodref_info {
                                class_index,
                                name_and_type_index,
                            },
                        )))
                    } else {
                        Err("Invalid references")
                    }
                },
            ),
            map_res(preceded(tag([8]), be_u16), |string_index| {
                if let Some(ConstantType::Utf8_info(string_index)) = pool.get_const(string_index) {
                    Ok(ConstantType::String_info(ConstValue::new(
                        0,
                        String_info { string_index },
                    )))
                } else {
                    Err("Invalid references")
                }
            }),
            map_res(preceded(tag([3]), be_u32), |bytes| {
                Ok::<ConstantType, &'static str>(ConstantType::Integer_info(ConstValue::new(
                    0,
                    Integer_info { bytes },
                )))
            }),
            map_res(preceded(tag([4]), float), |bytes| {
                Ok::<ConstantType, &'static str>(ConstantType::Float_info(ConstValue::new(
                    0,
                    Float_info { bytes },
                )))
            }),
            map_res(preceded(tag([5]), be_u64), |bytes| {
                Ok::<ConstantType, &'static str>(ConstantType::Long_info(ConstValue::new(
                    0,
                    Long_info { bytes },
                )))
            }),
            map_res(preceded(tag([6]), double), |bytes| {
                Ok::<ConstantType, &'static str>(ConstantType::Double_info(ConstValue::new(
                    0,
                    Double_info { bytes },
                )))
            }),
            map_res(
                preceded(tag([12]), tuple((be_u16, be_u16))),
                |(name_index, descriptor_index)| {
                    if let (
                        Some(ConstantType::Utf8_info(name_index)),
                        Some(ConstantType::Utf8_info(descriptor_index)),
                    ) = (pool.get_const(name_index), pool.get_const(descriptor_index))
                    {
                        Ok(ConstantType::NameAndType_info(ConstValue::new(
                            0,
                            NameAndType_info {
                                name_index,
                                descriptor_index,
                            },
                        )))
                    } else {
                        Err("Invalid references")
                    }
                },
            ),
            map_res(preceded(tag([1]), length_data(be_u16)), |bytes: &[u8]| {
                Ok::<ConstantType, &'static str>(ConstantType::Utf8_info(ConstValue::new(
                    0,
                    Utf8_info {
                        bytes: bytes.to_vec(),
                    },
                )))
            }),
            map_res(
                preceded(tag([15]), tuple((be_u8, be_u16))),
                |(reference_kind, reference_index)| match (
                    reference_kind,
                    pool.get_const(reference_index),
                ) {
                    (1..=4, Some(ConstantType::Fieldref_info(reference_index))) => {
                        Ok(ConstantType::MethodHandle_info(ConstValue::new(
                            0,
                            MethodHandle_info {
                                reference_kind,
                                reference_index: MethodHandleType::Fieldref_info(reference_index),
                            },
                        )))
                    }
                    (5..=8, Some(ConstantType::Methodref_info(reference_index))) => {
                        Ok(ConstantType::MethodHandle_info(ConstValue::new(
                            0,
                            MethodHandle_info {
                                reference_kind,
                                reference_index: MethodHandleType::Methodref_info(reference_index),
                            },
                        )))
                    }
                    (9, Some(ConstantType::InterfaceMethodref_info(reference_index))) => {
                        Ok(ConstantType::MethodHandle_info(ConstValue::new(
                            0,
                            MethodHandle_info {
                                reference_kind,
                                reference_index: MethodHandleType::InterfaceMethodref_info(
                                    reference_index,
                                ),
                            },
                        )))
                    }
                    _ => Err("Invalid reference"),
                },
            ),
            map_res(preceded(tag([16]), be_u16), |descriptor_index| {
                if let Some(ConstantType::Utf8_info(descriptor_index)) =
                    pool.get_const(descriptor_index)
                {
                    Ok(ConstantType::MethodType_info(ConstValue::new(
                        0,
                        MethodType_info { descriptor_index },
                    )))
                } else {
                    Err("Invalid references")
                }
            }),
            map_res(
                preceded(tag([18]), tuple((be_u16, be_u16))),
                |(bootstrap_method_attr_index, name_and_type_index)| {
                    if let (Some(ConstantType::NameAndType_info(name_and_type_index)),) =
                        (pool.get_const(name_and_type_index),)
                    {
                        Ok(ConstantType::InvokeDynamic_info(ConstValue::new(
                            0,
                            InvokeDynamic_info {
                                bootstrap_method_attr_index,
                                name_and_type_index,
                            },
                        )))
                    } else {
                        Err("Invalid references")
                    }
                },
            ),
        ))(data)
    }
}
