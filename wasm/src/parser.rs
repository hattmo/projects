use std::{str, u8};

use nom::{
    branch::alt,
    bytes::streaming::tag,
    combinator::{map, map_res, value},
    error::{Error, ErrorKind},
    multi::{length_count, length_data, many0},
    number::{
        streaming::{self, u8 as byte},
        Endianness,
    },
    sequence::{delimited, pair, preceded, terminated, tuple},
    Err::Error as ErrType,
    IResult,
};

fn leb_u32(mut input: &[u8]) -> IResult<&[u8], u32> {
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

fn leb_u64(mut input: &[u8]) -> IResult<&[u8], u64> {
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

fn leb_i32(mut input: &[u8]) -> IResult<&[u8], i32> {
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

fn leb_i64(mut input: &[u8]) -> IResult<&[u8], i64> {
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

fn f32(input: &[u8]) -> IResult<&[u8], f32> {
    streaming::f32(Endianness::Little)(input)
}
fn f64(input: &[u8]) -> IResult<&[u8], f64> {
    streaming::f64(Endianness::Little)(input)
}

fn name(input: &[u8]) -> IResult<&[u8], &str> {
    map_res(length_data(leb_u32), str::from_utf8)(input)
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum NumberType {
    i32,
    i64,
    f32,
    f64,
}

fn number_type(input: &[u8]) -> IResult<&[u8], NumberType> {
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

fn vector_type(input: &[u8]) -> IResult<&[u8], VectorType> {
    value(VectorType::v128, tag(b"\x7b"))(input)
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum ReferenceType {
    funcref,
    externref,
}

fn reference_type(input: &[u8]) -> IResult<&[u8], ReferenceType> {
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

fn value_type(input: &[u8]) -> IResult<&[u8], ValueType> {
    alt((
        map(number_type, |v| ValueType::numtype(v)),
        map(vector_type, |v| ValueType::vectype(v)),
        map(reference_type, |v| ValueType::reftype(v)),
    ))(input)
}

pub struct ResultType(Vec<ValueType>);

fn result_type(input: &[u8]) -> IResult<&[u8], ResultType> {
    map(length_count(leb_u32, value_type), |values| {
        ResultType(values)
    })(input)
}

pub struct FuntionType {
    params: ResultType,
    results: ResultType,
}

fn function_type(input: &[u8]) -> IResult<&[u8], FuntionType> {
    map(
        preceded(tag(b"\x60"), tuple((result_type, result_type))),
        |(params, results)| FuntionType { params, results },
    )(input)
}

pub enum Limit {
    Bound(u32, u32),
    Unbound(u32),
}

fn limit(input: &[u8]) -> IResult<&[u8], Limit> {
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

fn memory_type(input: &[u8]) -> IResult<&[u8], MemoryType> {
    map(limit, |limit| MemoryType { limit })(input)
}

pub struct TableType {
    lim: Limit,
    et: ReferenceType,
}

fn table_type(input: &[u8]) -> IResult<&[u8], TableType> {
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

fn mutable(input: &[u8]) -> IResult<&[u8], Mut> {
    alt((
        value(Mut::Const, tag(b"\x00")),
        value(Mut::Var, tag(b"\x01")),
    ))(input)
}

pub struct GlobalType {
    m: Mut,
    t: ValueType,
}

fn global_type(input: &[u8]) -> IResult<&[u8], GlobalType> {
    map(tuple((value_type, mutable)), |(t, m)| GlobalType { m, t })(input)
}

pub enum Instruction {
    Control(ControlInstr),
    Reference(ReferenceInstr),
    Parametric(ParametricInstr),
    Variable(VariableInstr),
    Table(TableInstr),
    Memory(MemoryInstr),
    Numeric(NumericInstr),
    Vector(VectorInstr),
}

fn instruction(_input: &[u8]) -> IResult<&[u8], Instruction> {
    todo!()
}

pub enum BlockType {
    Empty,
    Val(ValueType),
    Index(TypeIdx),
}

fn block_type(_input: &[u8]) -> IResult<&[u8], BlockType> {
    todo!()
}

pub enum ControlInstr {
    Unreachable,
    Nop,
    Block(BlockType, Vec<Instruction>),
    Loop(BlockType, Vec<Instruction>),
    If(BlockType, Vec<Instruction>),
    IfElse(BlockType, Vec<Instruction>, Vec<Instruction>),
    Branch(LableIdx),
    BranchIf(LableIdx),
    BranchTable(Vec<LableIdx>, LableIdx),
    Return,
    Call(FuncIdx),
    CallIndirect(TypeIdx, TableIdx),
}

fn control_instr(input: &[u8]) -> IResult<&[u8], ControlInstr> {
    alt((
        map(tag(b"\x00"), |_| ControlInstr::Unreachable),
        map(tag(b"\x01"), |_| ControlInstr::Nop),
        map(
            delimited(
                tag(b"\x02"),
                pair(block_type, many0(instruction)),
                tag(b"\x0B"),
            ),
            |(bt, instr)| ControlInstr::Block(bt, instr),
        ),
        map(
            delimited(
                tag(b"\x03"),
                pair(block_type, many0(instruction)),
                tag(b"\x0B"),
            ),
            |(bt, instr)| ControlInstr::Loop(bt, instr),
        ),
        map(
            delimited(
                tag(b"\x04"),
                pair(block_type, many0(instruction)),
                tag(b"\x0B"),
            ),
            |(bt, instr)| ControlInstr::If(bt, instr),
        ),
        map(
            tuple((
                tag(b"\x04"),
                block_type,
                many0(instruction),
                tag(b"\x05"),
                many0(instruction),
                tag(b"\x0B"),
            )),
            |(_, bt, if_instr, _, else_instr, _)| ControlInstr::IfElse(bt, if_instr, else_instr),
        ),
        map(preceded(tag(b"\x0C"), leb_u32), |l| {
            ControlInstr::Branch(LableIdx(l))
        }),
        map(preceded(tag(b"\x0D"), leb_u32), |l| {
            ControlInstr::BranchIf(LableIdx(l))
        }),
        map(
            preceded(
                tag(b"\x0E"),
                pair(
                    length_count(leb_u32, map(leb_u32, |l| LableIdx(l))),
                    leb_u32,
                ),
            ),
            |(l, ln)| ControlInstr::BranchTable(l, LableIdx(ln)),
        ),
        map(tag(b"\x0F"), |_| ControlInstr::Return),
        map(preceded(tag(b"\x10"), leb_u32), |x| {
            ControlInstr::Call(FuncIdx(x))
        }),
        map(preceded(tag(b"\x10"), pair(leb_u32, leb_u32)), |(x, y)| {
            ControlInstr::CallIndirect(TypeIdx(x), TableIdx(y))
        }),
    ))(input)
}

#[derive(Clone)]
pub enum ReferenceInstr {
    Null(ReferenceType),
    IsNull,
    Func(FuncIdx),
}

fn reference_instr(input: &[u8]) -> IResult<&[u8], ReferenceInstr> {
    alt((
        map(preceded(tag(b"\xD0"), reference_type), |t| {
            ReferenceInstr::Null(t)
        }),
        value(ReferenceInstr::IsNull, tag(b"\xD1")),
        map(preceded(tag(b"\xD2"), leb_u32), |x| {
            ReferenceInstr::Func(FuncIdx(x))
        }),
    ))(input)
}

pub enum ParametricInstr {
    Drop,
    Select,
    SelectMulti(Vec<ValueType>),
}

fn parametric_instr(input: &[u8]) -> IResult<&[u8], ParametricInstr> {
    alt((
        map(tag(b"\x1A"), |_| ParametricInstr::Drop),
        map(tag(b"\x1B"), |_| ParametricInstr::Select),
        map(
            preceded(tag(b"\x1C"), length_count(leb_u32, value_type)),
            |t| ParametricInstr::SelectMulti(t),
        ),
    ))(input)
}

pub enum VariableInstr {
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),
}

fn variable_instr(input: &[u8]) -> IResult<&[u8], VariableInstr> {
    alt((
        map(preceded(tag(b"\x20"), leb_u32), |x| {
            VariableInstr::LocalGet(LocalIdx(x))
        }),
        map(preceded(tag(b"\x21"), leb_u32), |x| {
            VariableInstr::LocalSet(LocalIdx(x))
        }),
        map(preceded(tag(b"\x22"), leb_u32), |x| {
            VariableInstr::LocalTee(LocalIdx(x))
        }),
        map(preceded(tag(b"\x23"), leb_u32), |x| {
            VariableInstr::GlobalGet(GlobalIdx(x))
        }),
        map(preceded(tag(b"\x24"), leb_u32), |x| {
            VariableInstr::GlobalSet(GlobalIdx(x))
        }),
    ))(input)
}

pub enum TableInstr {
    Get(TableIdx),
    Set(TableIdx),
    Init(ElemIdx, TableIdx),
    Drop(ElemIdx),
    Copy(TableIdx, TableIdx),
    Grow(TableIdx),
    Size(TableIdx),
    Fill(TableIdx),
}

fn table_instr(input: &[u8]) -> IResult<&[u8], TableInstr> {
    alt((
        map(preceded(tag(b"\x25"), leb_u32), |x| {
            TableInstr::Get(TableIdx(x))
        }),
        map(preceded(tag(b"\x26"), leb_u32), |x| {
            TableInstr::Set(TableIdx(x))
        }),
        table_instr_ext,
    ))(input)
}

fn table_instr_ext(input: &[u8]) -> IResult<&[u8], TableInstr> {
    let (input, ext) = preceded(tag(b"\xFC"), leb_u32)(input)?;
    match ext {
        12 => map(pair(leb_u32, leb_u32), |(x, y)| {
            TableInstr::Init(ElemIdx(x), TableIdx(y))
        })(input),
        13 => map(leb_u32, |x| TableInstr::Drop(ElemIdx(x)))(input),
        14 => map(pair(leb_u32, leb_u32), |(x, y)| {
            TableInstr::Copy(TableIdx(x), TableIdx(y))
        })(input),
        15 => map(leb_u32, |x| TableInstr::Grow(TableIdx(x)))(input),
        16 => map(leb_u32, |x| TableInstr::Size(TableIdx(x)))(input),
        17 => map(leb_u32, |x| TableInstr::Fill(TableIdx(x)))(input),
        _ => Err(ErrType(Error {
            input,
            code: ErrorKind::Fail,
        })),
    }
}
pub struct MemArg {
    align: u32,
    offset: u32,
}

fn mem_arg(_input: &[u8]) -> IResult<&[u8], MemArg> {
    todo!()
}

pub enum MemoryInstr {
    I32Load(MemArg),
    I64Load(MemArg),
    F32Load(MemArg),
    F64Load(MemArg),
    I32Load8s(MemArg),
    I32Load8u(MemArg),
    I32Load16s(MemArg),
    I32Load16u(MemArg),
    I64Load8s(MemArg),
    I64Load8u(MemArg),
    I64Load16s(MemArg),
    I64Load16u(MemArg),
    I64Load32s(MemArg),
    I64Load32u(MemArg),
    I32Store(MemArg),
    I64Store(MemArg),
    F32Store(MemArg),
    F64Store(MemArg),
    I32Store8(MemArg),
    I32Store16(MemArg),
    I64Store8(MemArg),
    I64Store16(MemArg),
    I64Store32(MemArg),
    MemorySize,
    MemoryGrow,
    MemoryInit(DataIdx),
    DataDrop(DataIdx),
    MemoryCopy,
    MemoryFill,
}

fn memory_instr(input: &[u8]) -> IResult<&[u8], MemoryInstr> {
    alt((memory_instr_load, memory_instr_store))(input)
}

fn memory_instr_load(input: &[u8]) -> IResult<&[u8], MemoryInstr> {
    alt((
        map(preceded(tag(b"\x28"), mem_arg), |m| MemoryInstr::I32Load(m)),
        map(preceded(tag(b"\x29"), mem_arg), |m| MemoryInstr::I64Load(m)),
        map(preceded(tag(b"\x2A"), mem_arg), |m| MemoryInstr::F32Load(m)),
        map(preceded(tag(b"\x2B"), mem_arg), |m| MemoryInstr::F64Load(m)),
        map(preceded(tag(b"\x2C"), mem_arg), |m| {
            MemoryInstr::I32Load8s(m)
        }),
        map(preceded(tag(b"\x2D"), mem_arg), |m| {
            MemoryInstr::I32Load8u(m)
        }),
        map(preceded(tag(b"\x2E"), mem_arg), |m| {
            MemoryInstr::I32Load16s(m)
        }),
        map(preceded(tag(b"\x2F"), mem_arg), |m| {
            MemoryInstr::I32Load16u(m)
        }),
        map(preceded(tag(b"\x30"), mem_arg), |m| {
            MemoryInstr::I64Load8s(m)
        }),
        map(preceded(tag(b"\x31"), mem_arg), |m| {
            MemoryInstr::I64Load8u(m)
        }),
        map(preceded(tag(b"\x32"), mem_arg), |m| {
            MemoryInstr::I64Load16s(m)
        }),
        map(preceded(tag(b"\x33"), mem_arg), |m| {
            MemoryInstr::I64Load16u(m)
        }),
        map(preceded(tag(b"\x34"), mem_arg), |m| {
            MemoryInstr::I64Load32s(m)
        }),
        map(preceded(tag(b"\x35"), mem_arg), |m| {
            MemoryInstr::I64Load32u(m)
        }),
    ))(input)
}

fn memory_instr_store(input: &[u8]) -> IResult<&[u8], MemoryInstr> {
    alt((
        map(preceded(tag(b"\x36"), mem_arg), |m| {
            MemoryInstr::I32Store(m)
        }),
        map(preceded(tag(b"\x37"), mem_arg), |m| {
            MemoryInstr::I64Store(m)
        }),
        map(preceded(tag(b"\x38"), mem_arg), |m| {
            MemoryInstr::F32Store(m)
        }),
        map(preceded(tag(b"\x39"), mem_arg), |m| {
            MemoryInstr::F64Store(m)
        }),
        map(preceded(tag(b"\x3A"), mem_arg), |m| {
            MemoryInstr::I32Store8(m)
        }),
        map(preceded(tag(b"\x3B"), mem_arg), |m| {
            MemoryInstr::I32Store16(m)
        }),
        map(preceded(tag(b"\x3C"), mem_arg), |m| {
            MemoryInstr::I64Store8(m)
        }),
        map(preceded(tag(b"\x3D"), mem_arg), |m| {
            MemoryInstr::I64Store16(m)
        }),
        map(preceded(tag(b"\x3E"), mem_arg), |m| {
            MemoryInstr::I64Store32(m)
        }),
    ))(input)
}

fn memory_instr_ctrl(input: &[u8]) -> IResult<&[u8], MemoryInstr> {
    alt((
        map(tag(b"\x3F\x00"), |_| MemoryInstr::MemorySize),
        map(tag(b"\x40\x00"), |_| MemoryInstr::MemoryGrow),
    ))(input)
}

fn memory_instr_ext(input: &[u8]) -> IResult<&[u8], MemoryInstr> {
    let (input, ext) = preceded(tag(b"\xFC"), leb_u32)(input)?;
    match ext {
        8 => map(terminated(leb_u32, tag(b"\x00")), |x| {
            MemoryInstr::MemoryInit(DataIdx(x))
        })(input),
        9 => map(leb_u32, |x| MemoryInstr::DataDrop(DataIdx(x)))(input),
        10 => map(tag(b"\x00\x00"), |_| MemoryInstr::MemoryCopy)(input),
        11 => map(tag(b"\x00"), |_| MemoryInstr::MemoryFill)(input),
        _ => Err(ErrType(Error {
            input,
            code: ErrorKind::Fail,
        })),
    }
}
#[derive(Clone)]
pub enum NumericInstr {
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),

    I32Eqz,
    I32Eq,
    I32Ne,
    I32Lts,
    I32Ltu,
    I32Gts,
    I32Gtu,
    I32Les,
    I32Leu,
    I32Ges,
    I32Geu,

    I64Eqz,
    I64Eq,
    I64Ne,
    I64Lts,
    I64Ltu,
    I64Gts,
    I64Gtu,
    I64Les,
    I64Leu,
    I64Ges,
    I64Geu,

    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,

    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,

    I32Clz,
    I32Ctz,
    I32PopCnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32Divs,
    I32Divu,
    I32Rems,
    I32Remu,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32Shrs,
    I32Shru,
    I32Rotl,
    I32Rotr,

    I64Clz,
    I64Ctz,
    I64PopCnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64Divs,
    I64Divu,
    I64Rems,
    I64Remu,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64Shrs,
    I64Shru,
    I64Rotl,
    I64Rotr,

    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32CopySign,

    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64CopySign,

    I32WrapI64,
    I32TruncF32s,
    I32TruncF32u,
    I32TruncF64s,
    I32TruncF64u,
    I64ExtendI32s,
    I64ExtendI32u,
    I64TruncF32s,
    I64TruncF32u,
    I64TruncF64s,
    I64TruncF64u,
    F32ConvertI32s,
    F32ConvertI32u,
    F32ConvertI64s,
    F32ConvertI64u,
    F32DemoteF64,
    F64ConvertI32s,
    F64ConvertI32u,
    F64ConvertI64s,
    F64ConvertI64u,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,

    I32Extend8s,
    I32Extend16s,
    I64Extend8s,
    I64Extend16s,
    I64Extend32s,

    I32TruncSatF32s,
    I32TruncSatF32u,
    I32TruncSatF64s,
    I32TruncSatF64u,
    I64TruncSatF32s,
    I64TruncSatF32u,
    I64TruncSatF64s,
    I64TruncSatF64u,
}

fn numeric_instr(input: &[u8]) -> IResult<&[u8], NumericInstr> {
    alt((
        numeric_instr_const,
        numeric_instr_cmp_i32,
        numeric_instr_cmp_i64,
        numeric_instr_cmp_f32,
        numeric_instr_cmp_f64,
        numeric_instr_ops_i32,
        numeric_instr_ops_i64,
        numeric_instr_ops_f32,
        numeric_instr_ops_f64,
        numeric_instr_conv,
        numeric_instr_extend,
        numeric_instr_ext,
    ))(input)
}

fn numeric_instr_const(input: &[u8]) -> IResult<&[u8], NumericInstr> {
    alt((
        map(preceded(tag(b"\x41"), leb_i32), |n| {
            NumericInstr::I32Const(n)
        }),
        map(preceded(tag(b"\x42"), leb_i64), |n| {
            NumericInstr::I64Const(n)
        }),
        map(preceded(tag(b"\x43"), f32), |z| NumericInstr::F32Const(z)),
        map(preceded(tag(b"\x44"), f64), |z| NumericInstr::F64Const(z)),
    ))(input)
}

fn numeric_instr_cmp_i32(input: &[u8]) -> IResult<&[u8], NumericInstr> {
    alt((
        value(NumericInstr::I32Eqz, tag(b"\x45")),
        value(NumericInstr::I32Eq, tag(b"\x46")),
        value(NumericInstr::I32Ne, tag(b"\x47")),
        value(NumericInstr::I32Lts, tag(b"\x48")),
        value(NumericInstr::I32Ltu, tag(b"\x49")),
        value(NumericInstr::I32Gts, tag(b"\x4A")),
        value(NumericInstr::I32Gtu, tag(b"\x4B")),
        value(NumericInstr::I32Les, tag(b"\x4C")),
        value(NumericInstr::I32Leu, tag(b"\x4D")),
        value(NumericInstr::I32Ges, tag(b"\x4E")),
        value(NumericInstr::I32Geu, tag(b"\x4F")),
    ))(input)
}

fn numeric_instr_cmp_i64(input: &[u8]) -> IResult<&[u8], NumericInstr> {
    alt((
        value(NumericInstr::I64Eqz, tag(b"\x50")),
        value(NumericInstr::I64Eq, tag(b"\x51")),
        value(NumericInstr::I64Ne, tag(b"\x52")),
        value(NumericInstr::I64Lts, tag(b"\x53")),
        value(NumericInstr::I64Ltu, tag(b"\x54")),
        value(NumericInstr::I64Gts, tag(b"\x55")),
        value(NumericInstr::I64Gtu, tag(b"\x56")),
        value(NumericInstr::I64Les, tag(b"\x57")),
        value(NumericInstr::I64Leu, tag(b"\x58")),
        value(NumericInstr::I64Ges, tag(b"\x59")),
        value(NumericInstr::I64Geu, tag(b"\x5A")),
    ))(input)
}

fn numeric_instr_cmp_f32(input: &[u8]) -> IResult<&[u8], NumericInstr> {
    alt((
        value(NumericInstr::F32Eq, tag(b"\x5B")),
        value(NumericInstr::F32Ne, tag(b"\x5C")),
        value(NumericInstr::F32Lt, tag(b"\x5D")),
        value(NumericInstr::F32Gt, tag(b"\x5E")),
        value(NumericInstr::F32Le, tag(b"\x5F")),
        value(NumericInstr::F32Ge, tag(b"\x60")),
    ))(input)
}
fn numeric_instr_cmp_f64(input: &[u8]) -> IResult<&[u8], NumericInstr> {
    alt((
        value(NumericInstr::F64Eq, tag(b"\x61")),
        value(NumericInstr::F64Ne, tag(b"\x62")),
        value(NumericInstr::F64Lt, tag(b"\x63")),
        value(NumericInstr::F64Gt, tag(b"\x64")),
        value(NumericInstr::F64Le, tag(b"\x65")),
        value(NumericInstr::F64Ge, tag(b"\x66")),
    ))(input)
}

fn numeric_instr_ops_i32(input: &[u8]) -> IResult<&[u8], NumericInstr> {
    alt((
        value(NumericInstr::I32Clz, tag(b"\x67")),
        value(NumericInstr::I32Ctz, tag(b"\x68")),
        value(NumericInstr::I32PopCnt, tag(b"\x69")),
        value(NumericInstr::I32Add, tag(b"\x6A")),
        value(NumericInstr::I32Sub, tag(b"\x6B")),
        value(NumericInstr::I32Mul, tag(b"\x6C")),
        value(NumericInstr::I32Divs, tag(b"\x6D")),
        value(NumericInstr::I32Divu, tag(b"\x6E")),
        value(NumericInstr::I32Rems, tag(b"\x6F")),
        value(NumericInstr::I32Remu, tag(b"\x70")),
        value(NumericInstr::I32And, tag(b"\x71")),
        value(NumericInstr::I32Or, tag(b"\x72")),
        value(NumericInstr::I32Xor, tag(b"\x73")),
        value(NumericInstr::I32Shl, tag(b"\x74")),
        value(NumericInstr::I32Shrs, tag(b"\x75")),
        value(NumericInstr::I32Shru, tag(b"\x76")),
        value(NumericInstr::I32Rotl, tag(b"\x77")),
        value(NumericInstr::I32Rotr, tag(b"\x78")),
    ))(input)
}

fn numeric_instr_ops_i64(input: &[u8]) -> IResult<&[u8], NumericInstr> {
    alt((
        value(NumericInstr::I64Clz, tag(b"\x79")),
        value(NumericInstr::I64Ctz, tag(b"\x7A")),
        value(NumericInstr::I64PopCnt, tag(b"\x7B")),
        value(NumericInstr::I64Add, tag(b"\x7C")),
        value(NumericInstr::I64Sub, tag(b"\x7D")),
        value(NumericInstr::I64Mul, tag(b"\x7E")),
        value(NumericInstr::I64Divs, tag(b"\x7F")),
        value(NumericInstr::I64Divu, tag(b"\x80")),
        value(NumericInstr::I64Rems, tag(b"\x81")),
        value(NumericInstr::I64Remu, tag(b"\x82")),
        value(NumericInstr::I64And, tag(b"\x83")),
        value(NumericInstr::I64Or, tag(b"\x84")),
        value(NumericInstr::I64Xor, tag(b"\x85")),
        value(NumericInstr::I64Shl, tag(b"\x86")),
        value(NumericInstr::I64Shrs, tag(b"\x87")),
        value(NumericInstr::I64Shru, tag(b"\x88")),
        value(NumericInstr::I64Rotl, tag(b"\x89")),
        value(NumericInstr::I64Rotr, tag(b"\x8A")),
    ))(input)
}

fn numeric_instr_ops_f32(input: &[u8]) -> IResult<&[u8], NumericInstr> {
    alt((
        value(NumericInstr::F32Abs, tag(b"\x8B")),
        value(NumericInstr::F32Neg, tag(b"\x8C")),
        value(NumericInstr::F32Ceil, tag(b"\x8D")),
        value(NumericInstr::F32Floor, tag(b"\x8E")),
        value(NumericInstr::F32Trunc, tag(b"\x8F")),
        value(NumericInstr::F32Nearest, tag(b"\x90")),
        value(NumericInstr::F32Sqrt, tag(b"\x91")),
        value(NumericInstr::F32Add, tag(b"\x92")),
        value(NumericInstr::F32Sub, tag(b"\x93")),
        value(NumericInstr::F32Mul, tag(b"\x94")),
        value(NumericInstr::F32Div, tag(b"\x95")),
        value(NumericInstr::F32Min, tag(b"\x96")),
        value(NumericInstr::F32Max, tag(b"\x97")),
        value(NumericInstr::F32CopySign, tag(b"\x98")),
    ))(input)
}

fn numeric_instr_ops_f64(input: &[u8]) -> IResult<&[u8], NumericInstr> {
    alt((
        value(NumericInstr::F64Abs, tag(b"\x99")),
        value(NumericInstr::F64Neg, tag(b"\x9A")),
        value(NumericInstr::F64Ceil, tag(b"\x9B")),
        value(NumericInstr::F64Floor, tag(b"\x9C")),
        value(NumericInstr::F64Trunc, tag(b"\x9D")),
        value(NumericInstr::F64Nearest, tag(b"\x9E")),
        value(NumericInstr::F64Sqrt, tag(b"\x9F")),
        value(NumericInstr::F64Add, tag(b"\xA0")),
        value(NumericInstr::F64Sub, tag(b"\xA1")),
        value(NumericInstr::F64Mul, tag(b"\xA2")),
        value(NumericInstr::F64Div, tag(b"\xA3")),
        value(NumericInstr::F64Min, tag(b"\xA4")),
        value(NumericInstr::F64Max, tag(b"\xA5")),
        value(NumericInstr::F64CopySign, tag(b"\xA6")),
    ))(input)
}

fn numeric_instr_conv(input: &[u8]) -> IResult<&[u8], NumericInstr> {
    alt((
        value(NumericInstr::I32WrapI64, tag(b"\xA7")),
        alt((
            value(NumericInstr::I32TruncF32s, tag(b"\xA8")),
            value(NumericInstr::I32TruncF32u, tag(b"\xA9")),
            value(NumericInstr::I32TruncF64s, tag(b"\xAA")),
            value(NumericInstr::I32TruncF64u, tag(b"\xAB")),
        )),
        alt((
            value(NumericInstr::I64ExtendI32s, tag(b"\xAC")),
            value(NumericInstr::I64ExtendI32u, tag(b"\xAD")),
        )),
        alt((
            value(NumericInstr::I64TruncF32s, tag(b"\xAE")),
            value(NumericInstr::I64TruncF32u, tag(b"\xAF")),
            value(NumericInstr::I64TruncF64s, tag(b"\xB0")),
            value(NumericInstr::I64TruncF64u, tag(b"\xB1")),
        )),
        alt((
            value(NumericInstr::F32ConvertI32s, tag(b"\xB2")),
            value(NumericInstr::F32ConvertI32u, tag(b"\xB3")),
            value(NumericInstr::F32ConvertI64s, tag(b"\xB4")),
            value(NumericInstr::F32ConvertI64u, tag(b"\xB5")),
        )),
        value(NumericInstr::F32DemoteF64, tag(b"\xB6")),
        alt((
            value(NumericInstr::F64ConvertI32s, tag(b"\xB7")),
            value(NumericInstr::F64ConvertI32u, tag(b"\xB8")),
            value(NumericInstr::F64ConvertI64s, tag(b"\xB9")),
            value(NumericInstr::F64ConvertI64u, tag(b"\xBA")),
        )),
        value(NumericInstr::F64PromoteF32, tag(b"\xBB")),
        alt((
            value(NumericInstr::I32ReinterpretF32, tag(b"\xBC")),
            value(NumericInstr::I64ReinterpretF64, tag(b"\xBD")),
            value(NumericInstr::F32ReinterpretI32, tag(b"\xBE")),
            value(NumericInstr::F64ReinterpretI64, tag(b"\xBF")),
        )),
    ))(input)
}

fn numeric_instr_extend(input: &[u8]) -> IResult<&[u8], NumericInstr> {
    alt((
        value(NumericInstr::I32Extend8s, tag(b"\xBC")),
        value(NumericInstr::I32Extend16s, tag(b"\xBC")),
        value(NumericInstr::I64Extend8s, tag(b"\xBC")),
        value(NumericInstr::I64Extend16s, tag(b"\xBC")),
        value(NumericInstr::I64Extend32s, tag(b"\xBC")),
    ))(input)
}

fn numeric_instr_ext(input: &[u8]) -> IResult<&[u8], NumericInstr> {
    let (input, ext) = preceded(tag(b"\xFC"), leb_u32)(input)?;
    match ext {
        0 => Ok((input, NumericInstr::I32TruncSatF32s)),
        1 => Ok((input, NumericInstr::I32TruncSatF32u)),
        2 => Ok((input, NumericInstr::I32TruncSatF64s)),
        3 => Ok((input, NumericInstr::I32TruncSatF64u)),
        4 => Ok((input, NumericInstr::I64TruncSatF32s)),
        5 => Ok((input, NumericInstr::I64TruncSatF32u)),
        6 => Ok((input, NumericInstr::I64TruncSatF64s)),
        7 => Ok((input, NumericInstr::I64TruncSatF64u)),
        _ => Err(ErrType(Error {
            input,
            code: ErrorKind::Fail,
        })),
    }
}

pub struct LaneIdx(u8);

pub enum VectorInstr {
    V128Load(MemArg),
    V128Load8X8s(MemArg),
    V128Load8X8u(MemArg),
    V128Load16X4s(MemArg),
    V128Load16X4u(MemArg),
    V128Load32X2s(MemArg),
    V128Load32X2u(MemArg),
    V128Load8Splat(MemArg),
    V128Load16Splat(MemArg),
    V128Load32Splat(MemArg),
    V128Load64Splat(MemArg),
    V128Load32Zero(MemArg),
    V128Load64Zero(MemArg),
    V128Store(MemArg),
    V128Load8Lane(MemArg, LaneIdx),
    V128Load16Lane(MemArg, LaneIdx),
    V128Load32Lane(MemArg, LaneIdx),
    V128Load64Lane(MemArg, LaneIdx),
    V128Store8Lane(MemArg, LaneIdx),
    V128Store16Lane(MemArg, LaneIdx),
    V128Store32Lane(MemArg, LaneIdx),
    V128Store64Lane(MemArg, LaneIdx),

    V128Const(i128),

    I8X16Shuffle([LaneIdx; 16]),

    I8X16ExtractLanes(LaneIdx),
    I8X16ExtractLaneu(LaneIdx),
    I8X16ReplaceLane(LaneIdx),
    I16X8ExtractLanes(LaneIdx),
    I16X8ExtractLaneu(LaneIdx),
    I16X8ReplaceLane(LaneIdx),
    I32X4ExtractLane(LaneIdx),
    I32X4ReplaceLane(LaneIdx),
    I64X2ExtractLane(LaneIdx),
    I64X2ReplaceLane(LaneIdx),
    F32X4ExtractLane(LaneIdx),
    F32X4ReplaceLane(LaneIdx),
    F64X2ExtractLane(LaneIdx),
    F64X2ReplaceLane(LaneIdx),
}

#[derive(Clone)]
pub struct TypeIdx(u32);
#[derive(Clone)]
pub struct FuncIdx(u32);
#[derive(Clone)]
pub struct TableIdx(u32);
#[derive(Clone)]
pub struct MemIdx(u32);
#[derive(Clone)]
pub struct GlobalIdx(u32);
#[derive(Clone)]
pub struct ElemIdx(u32);
#[derive(Clone)]
pub struct DataIdx(u32);
#[derive(Clone)]
pub struct LocalIdx(u32);
#[derive(Clone)]
pub struct LableIdx(u32);

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
