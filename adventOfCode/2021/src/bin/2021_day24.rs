use std::{collections::HashMap, error::Error, fmt::Debug, hash::DefaultHasher};

fn main() -> Result<(), Box<dyn Error>> {
    let input = helper::get_input("2021", "24", true)?;
    let res = part1(&input);
    println!("part1: {res:?}");
    let res = part2(&input);
    println!("part2: {res:?}");
    Ok(())
}
#[derive(Hash, PartialEq, Eq)]
enum Reg {
    W(usize),
    X(usize),
    Y(usize),
    Z(usize),
}

trait AsReg {
    fn as_reg(&self, c: &Counts) -> Reg;
    fn as_dst_reg(&self, c: &mut Counts) -> Reg;
}

impl<T> AsReg for T
where
    T: AsRef<str>,
{
    fn as_dst_reg(&self, c: &mut Counts) -> Reg {
        match self.as_ref() {
            "w" => {
                c.w += 1;
                Reg::W(c.w)
            }
            "x" => {
                c.x += 1;
                Reg::X(c.x)
            }
            "y" => {
                c.y += 1;
                Reg::Y(c.y)
            }
            "z" => {
                c.z += 1;
                Reg::Z(c.z)
            }
            _ => panic!(),
        }
    }

    fn as_reg(&self, c: &Counts) -> Reg {
        match self.as_ref() {
            "w" => Reg::W(c.w),
            "x" => Reg::X(c.x),
            "y" => Reg::Y(c.y),
            "z" => Reg::Z(c.z),
            _ => panic!(),
        }
    }
}

enum Param {
    Reg(Reg),
    Imm(isize),
}

enum Op {
    Inp(usize),
    Add(Reg, Param),
    Mul(Reg, Param),
    Div(Reg, Param),
    Mod(Reg, Param),
    Eql(Reg, Param),
}

struct Counts {
    w: usize,
    x: usize,
    y: usize,
    z: usize,
}
impl Default for Counts {
    fn default() -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

fn parse_input(input: &str) -> HashMap<Reg, Op> {
    let mut i_count = 0;
    let mut counts = Counts::default();
    input
        .lines()
        .map(|line| {
            let parts = line.split(" ").map(|i| i.trim()).collect::<Vec<_>>();
            match parts.as_slice() {
                ["inp", reg] => {
                    i_count += 1;
                    (reg.as_dst_reg(&mut counts), Op::Inp(i_count))
                }
                ["add", reg, param] => {}
                _ => panic!(),
            }
        })
        .collect()
}

fn part1(input: &str) -> impl Debug + use<'_> {
    parse_input(input);
    "todo"
}
fn part2(_input: &str) -> impl Debug + use<'_> {
    "todo"
}
