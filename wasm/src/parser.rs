use std::{alloc::Global, u8};

use nom::{
    branch::alt,
    bytes::streaming::tag,
    combinator::{map, map_res, value},
    multi::{length_count, length_data},
    number::{streaming, Endianness},
    sequence::{preceded, tuple},
    IResult,
};

pub fn byte(input: &[u8]) -> IResult<&[u8], u8> {
    streaming::u8(input)
}

pub fn leb_u32(mut input: &[u8]) -> IResult<&[u8], u32> {
    let mut out = 0;
    let mut off = 0;
    loop {
        let (rem, b) = byte(input)?;
        input = rem;
        out |= ((b & 127) as u32) << off;
        if b < 127 {
            break;
        };
        off += 7;
    }
    Ok((input, out))
}

pub fn leb_u64(mut input: &[u8]) -> IResult<&[u8], u64> {
    let mut out = 0;
    let mut off = 0;
    loop {
        let (rem, b) = byte(input)?;
        input = rem;
        out |= ((b & 127) as u64) << off;
        if b < 127 {
            break;
        };
        off += 7;
    }
    Ok((input, out))
}

pub fn leb_i32(mut input: &[u8]) -> IResult<&[u8], i32> {
    let size = 32;
    let mut out = 0;
    let mut off = 0;
    loop {
        let (rem, b) = byte(input)?;
        input = rem;
        out |= ((b & 127) as i32) << off;
        off += 7;
        if b < 127 {
            if (off < size) && (b & 0x40 > 0) {
                out |= !0 << off;
            }
            break;
        }
    }
    Ok((input, out))
}

pub fn leb_i64(mut input: &[u8]) -> IResult<&[u8], i64> {
    let size = 64;
    let mut out = 0;
    let mut off = 0;
    loop {
        let (rem, b) = byte(input)?;
        input = rem;
        out |= ((b & 127) as i64) << off;
        off += 7;
        if b < 127 {
            if (off < size) && (b & 0x40 > 0) {
                out |= !0 << off;
            }
            break;
        }
    }
    Ok((input, out))
}

pub fn f32(input: &[u8]) -> IResult<&[u8], f32> {
    streaming::f32(Endianness::Little)(input)
}
pub fn f64(input: &[u8]) -> IResult<&[u8], f64> {
    streaming::f64(Endianness::Little)(input)
}

pub fn name(input: &[u8]) -> IResult<&[u8], &str> {
    map_res(length_data(leb_u32), std::str::from_utf8)(input)
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum NumberType {
    i32,
    i64,
    f32,
    f64,
}

pub fn number_type(input: &[u8]) -> IResult<&[u8], NumberType> {
    alt((
        value(NumberType::i32, tag(b"\x7F")),
        value(NumberType::i64, tag(b"\x7E")),
        value(NumberType::f32, tag(b"\x7D")),
        value(NumberType::f64, tag(b"\x7C")),
    ))(input)
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum VectorType {
    v128,
}

pub fn vector_type(input: &[u8]) -> IResult<&[u8], VectorType> {
    value(VectorType::v128, tag(b"\x7b"))(input)
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum ReferenceType {
    funcref,
    externref,
}

pub fn reference_type(input: &[u8]) -> IResult<&[u8], ReferenceType> {
    alt((
        value(ReferenceType::funcref, tag(b"\x70")),
        value(ReferenceType::externref, tag(b"\x6E")),
    ))(input)
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum ValueType {
    numtype(NumberType),
    vectype(VectorType),
    reftype(ReferenceType),
}

pub fn value_type(input: &[u8]) -> IResult<&[u8], ValueType> {
    alt((
        map(number_type, |v| ValueType::numtype(v)),
        map(vector_type, |v| ValueType::vectype(v)),
        map(reference_type, |v| ValueType::reftype(v)),
    ))(input)
}

pub fn result_type(input: &[u8]) -> IResult<&[u8], Vec<ValueType>> {
    length_count(leb_u32, value_type)(input)
}

pub struct FuntionType {
    params: Vec<ValueType>,
    results: Vec<ValueType>,
}

pub fn function_type(input: &[u8]) -> IResult<&[u8], FuntionType> {
    map(
        preceded(tag(b"\x60"), tuple((result_type, result_type))),
        |(params, results)| FuntionType { params, results },
    )(input)
}

pub enum Limit {
    Bound(u32, u32),
    Unbound(u32),
}

pub fn limit(input: &[u8]) -> IResult<&[u8], Limit> {
    alt((
        map(preceded(tag(b"\x00"), leb_u32), |lower| {
            Limit::Unbound(lower)
        }),
        map(
            preceded(tag(b"\x01"), tuple((leb_u32, leb_u32))),
            |(lower, upper)| Limit::Bound(lower, upper),
        ),
    ))(input)
}

pub struct MemoryType {
    limit: Limit,
}

pub fn memory_type(input: &[u8]) -> IResult<&[u8], MemoryType> {
    map(limit, |limit| MemoryType { limit })(input)
}

pub struct TableType {
    lim: Limit,
    et: ReferenceType,
}

pub fn table_type(input: &[u8]) -> IResult<&[u8], TableType> {
    map(tuple((reference_type, limit)), |(et, lim)| TableType {
        lim,
        et,
    })(input)
}

#[derive(Clone)]
pub enum Mut {
    Const,
    Var,
}

pub fn mutable(input: &[u8]) -> IResult<&[u8], Mut> {
    alt((
        value(Mut::Const, tag(b"\x00")),
        value(Mut::Var, tag(b"\x01")),
    ))(input)
}

pub struct GlobalType {
    m: Mut,
    t: ValueType,
}

pub fn global_type(input: &[u8]) -> IResult<&[u8], GlobalType> {
    map(tuple((value_type, mutable)), |(t, m)| GlobalType { m, t })(input)
}

enum BlockType {
    Empty,
    Val(ValueType),
    Index(u32),
}

enum Instruction {
    Control(Control),
    Reference,
    Parametric,
    Variable,
    Table,
    Memory,
    Numeric,
    Vector,
}

pub fn instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
    todo!()
}

enum Control {
    Unreachable,
    Nop,
    Block(BlockType, Vec<Instruction>),
    Loop(BlockType, Vec<Instruction>),
    If(BlockType, Vec<Instruction>),
    IfElse(BlockType, Vec<Instruction>, Vec<Instruction>),
    Branch(u32),
    BranchIf(u32),
    BranchTable(Vec<u32>, u32),
    Return,
    Call(u32),
    CallIndirect(u32, u32),
}

pub fn control(input: &[u8]) -> IResult<&[u8], Control> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lebi32() {
        let test_data = [0xC0, 0xBB, 0x78];
        let (rem, res) = leb_i32(&test_data).unwrap();
        println!("RES: {res}");
        assert!(res == -123456);
        assert!(rem.len() == 0);
    }
    #[test]
    fn test_lebi64() {
        let test_data = [0xC0, 0xBB, 0x78];
        let (rem, res) = leb_i64(&test_data).unwrap();
        println!("RES: {res}");
        assert!(res == -123456);
        assert!(rem.len() == 0);
    }
}
