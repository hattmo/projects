use std::fmt::Debug;

fn main() {
    helper::run("2024", "3", part_1, part_2);
}

fn parse_mul(sym: &str) -> (usize, usize) {
    let (left, right) = sym.split_once(",").unwrap();
    let (_, left) = left.split_once("(").unwrap();
    let (right, _) = right.split_once(")").unwrap();
    let left: usize = left.parse().unwrap();
    let right: usize = right.parse().unwrap();
    (left, right)
}

fn part_1(input: &str) -> impl Debug {
    let rex = regex::Regex::new(r#"mul\(\d+,\d+\)"#).unwrap();
    let res: usize = input
        .lines()
        .map(|line| rex.find_iter(line).map(|item| item.as_str()))
        .flatten()
        .map(parse_mul)
        .map(|(left, right)| left * right)
        .sum();
    res
}
#[derive(Debug)]
enum Symbol {
    Do,
    Dont,
    Mul(usize, usize),
}

fn part_2(input: &str) -> impl Debug {
    let rex = regex::Regex::new(r#"(mul\(\d+,\d+\)|do\(\)|don't\(\))"#).unwrap();
    let matches = input
        .lines()
        .map(|line| rex.find_iter(line).map(|item| item.as_str()))
        .flatten()
        .map(|sym| match sym {
            "do()" => Symbol::Do,
            "don't()" => Symbol::Dont,
            mul => {
                let (left, right) = parse_mul(mul);
                Symbol::Mul(left, right)
            }
        });
    let mut on: bool = true;
    let mut total: usize = 0;
    for sym in matches {
        match (sym, on) {
            (Symbol::Do, false) => on = true,
            (Symbol::Dont, true) => on = false,
            (Symbol::Mul(left, right), true) => total += left * right,
            _ => (),
        }
    }
    total
}
