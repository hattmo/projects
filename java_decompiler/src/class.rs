use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::{count, length_data},
    number::{
        complete::{be_u16, be_u32, be_u64, be_u8, double},
        streaming::float,
    },
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
pub struct Class<'a> {
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<Constant<'a>>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    // interfaces: Vec<Interface>,
    // fields: Vec<Field>,
    // methods: Vec<Method>,
    // attributes: Vec<Attribute>,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Constant<'a> {
    Class(u16),
    Fieldref_info(u16, u16),
    Methodref_info(u16, u16),
    InterfaceMethodref_info(u16, u16),
    String_info(u16),
    Integer_info(u32),
    Float_info(f32),
    Long_info(u64),
    Double_info(f64),
    NameAndType_info(u16, u16),
    Utf8_info(&'a [u8]),
    MethodHandle_info(u8, u16),
    MethodType_info(u16),
    InvokeDynamic_info(u16, u16),
}

impl<'a> Class<'a> {
    pub fn from_slice(data: &[u8]) -> IResult<&[u8], Class> {
        let (data, (minor_version, major_version)) =
            preceded(tag(b"\xCA\xFE\xBA\xBE"), tuple((be_u16, be_u16)))(data)?;
        let (data, constant_pool) = Self::parse_const_pool(data)?;
        let (data, (access_flags, this_class, super_class)) =
            tuple((be_u16, be_u16, be_u16))(data)?;
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
    }
    fn parse_const_pool(data: &[u8]) -> IResult<&[u8], Vec<Constant>> {
        let (mut rest, const_count) = be_u16(data)?;
        let mut curr_index = 0;
        let mut out = Vec::new();
        while curr_index < (const_count - 1) {
            curr_index += 1;
            let (new_rest, const_item) = Self::parse_const_item(rest)?;
            rest = new_rest;
            println!("{:?}:{:?}", curr_index, const_item);
            if let Constant::Double_info(_) = const_item {
                curr_index += 1;
            } else if let Constant::Long_info(_) = const_item {
                curr_index += 1;
            }
            out.push(const_item);
        }
        Ok((rest, out))
    }
    fn parse_const_item(data: &[u8]) -> IResult<&[u8], Constant> {
        alt((
            map(preceded(tag([7]), be_u16), |name_index| {
                Constant::Class(name_index)
            }),
            map(
                preceded(tag([9]), tuple((be_u16, be_u16))),
                |(class_index, name_and_type_index)| {
                    Constant::Fieldref_info(class_index, name_and_type_index)
                },
            ),
            map(
                preceded(tag([10]), tuple((be_u16, be_u16))),
                |(class_index, name_and_type_index)| {
                    Constant::Methodref_info(class_index, name_and_type_index)
                },
            ),
            map(
                preceded(tag([11]), tuple((be_u16, be_u16))),
                |(class_index, name_and_type_index)| {
                    Constant::InterfaceMethodref_info(class_index, name_and_type_index)
                },
            ),
            map(preceded(tag([8]), be_u16), |string_index| {
                Constant::String_info(string_index)
            }),
            map(preceded(tag([3]), be_u32), |bytes| {
                Constant::Integer_info(bytes)
            }),
            map(preceded(tag([4]), float), |bytes| {
                Constant::Float_info(bytes)
            }),
            map(preceded(tag([5]), be_u64), |bytes| {
                Constant::Long_info(bytes)
            }),
            map(preceded(tag([6]), double), |bytes| {
                Constant::Double_info(bytes)
            }),
            map(
                preceded(tag([12]), tuple((be_u16, be_u16))),
                |(name_index, descriptor_index)| {
                    Constant::NameAndType_info(name_index, descriptor_index)
                },
            ),
            map(preceded(tag([1]), length_data(be_u16)), |bytes| {
                Constant::Utf8_info(bytes)
            }),
            map(
                preceded(tag([15]), tuple((be_u8, be_u16))),
                |(reference_kind, reference_index)| {
                    Constant::MethodHandle_info(reference_kind, reference_index)
                },
            ),
            map(preceded(tag([16]), be_u16), |descriptor_index| {
                Constant::MethodType_info(descriptor_index)
            }),
            map(
                preceded(tag([18]), tuple((be_u16, be_u16))),
                |(bootstrap_method_attr_index, name_and_type_index)| {
                    Constant::InvokeDynamic_info(bootstrap_method_attr_index, name_and_type_index)
                },
            ),
        ))(data)
    }
}

#[cfg(test)]
mod test {
    use std::io::Read;

    use super::Class;

    #[test]
    fn print_test() -> Result<(), std::io::Error> {
        let mut file = std::fs::File::open("test/Test.class")?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        let (_, class) = Class::from_slice(&buf).unwrap();
        print!("{:?}", class);
        Ok(())
    }
}
