use core::panic;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = helper::get_input("2024", "7", true)?;
    let res = part1(&input);
    println!("part1: {res:?}");
    let res = part2(&input);
    println!("part2: {res:?}");
    Ok(())
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

struct Equation(Vec<Operation>);
impl Equation {
    fn new(size: usize) -> Self {
        Self(vec![Operation::Add; size])
    }

    fn next(&mut self) -> bool {
        if !self.0.contains(&Operation::Add) {
            return true;
        }
        for op in self.0.iter_mut() {
            match op {
                Operation::Add => {
                    *op = Operation::Mul;
                    break;
                }
                Operation::Mul => {
                    *op = Operation::Add;
                }
            }
        }
        false
    }

    fn eval(&self, nums: &[usize]) -> usize {
        let [start, rest @ ..] = nums else {
            panic!("not enough numbers");
        };
        self.0
            .iter()
            .zip(rest)
            .fold(*start, |acc, (op, next)| match op {
                Operation::Add => acc + next,
                Operation::Mul => acc * next,
            })
    }
}

fn part1(input: &str) -> impl Debug + use<'_> {
    input
        .lines()
        .map(|line| {
            let (test, nums) = line.split_once(":").unwrap();
            let test = test.parse::<usize>().unwrap();
            let nums: Vec<_> = nums
                .split(" ")
                .filter(|item| !item.is_empty())
                .map(|num| num.trim().parse::<usize>().unwrap())
                .collect();
            (test, nums)
        })
        .filter_map(|(test, nums)| {
            let mut equation = Equation::new(nums.len() - 1);
            loop {
                let res = equation.eval(&nums);
                if res == test {
                    return Some(test);
                }
                let done = equation.next();
                if done {
                    return None;
                }
            }
        })
        .sum::<usize>()
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum OperationExt {
    Add,
    Mul,
    Concat,
}

impl Display for OperationExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationExt::Add => write!(f, "+"),
            OperationExt::Mul => write!(f, "*"),
            OperationExt::Concat => write!(f, "||"),
        }
    }
}

struct EquationExt(Vec<OperationExt>);
impl EquationExt {
    fn new(size: usize) -> Self {
        Self(vec![OperationExt::Add; size])
    }

    fn next(&mut self) -> bool {
        if self.0.iter().all(|item| item == &OperationExt::Concat) {
            return true;
        }
        for op in self.0.iter_mut() {
            match op {
                OperationExt::Add => {
                    *op = OperationExt::Mul;
                    break;
                }
                OperationExt::Mul => {
                    *op = OperationExt::Concat;
                    break;
                }
                OperationExt::Concat => {
                    *op = OperationExt::Add;
                }
            }
        }
        false
    }

    fn eval(&self, nums: &[u128]) -> u128 {
        let [start, rest @ ..] = nums else {
            panic!("not enough numbers");
        };
        self.0
            .iter()
            .zip(rest)
            .fold(*start, |acc, (op, next)| match op {
                OperationExt::Add => acc + next,
                OperationExt::Mul => acc * next,
                OperationExt::Concat => {
                    let mut acc_tmp = acc;
                    let mut next_tmp = *next;
                    while next_tmp != 0 {
                        acc_tmp *= 10;
                        next_tmp /= 10;
                    }
                    acc_tmp + next
                }
            })
    }
}

impl Display for EquationExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .0
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{out}")
    }
}

fn part2(input: &str) -> impl Debug + use<'_> {
    input
        .lines()
        .map(|line| {
            let (test, nums) = line.split_once(":").unwrap();
            let test = test.parse::<u128>().unwrap();
            let nums: Vec<_> = nums
                .split(" ")
                .filter(|item| !item.is_empty())
                .map(|num| num.trim().parse::<u128>().unwrap())
                .collect();
            (test, nums)
        })
        .filter_map(|(test, nums)| {
            let mut equation = EquationExt::new(nums.len() - 1);
            loop {
                let res = equation.eval(&nums);
                if res == test {
                    return Some(test);
                }
                let done = equation.next();
                if done {
                    return None;
                }
            }
        })
        .sum::<u128>()
}

#[cfg(test)]
mod test {
    use crate::part2;

    #[test]
    fn test_part2() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let res = part2(&input);
        println!("{res:?}");
    }
}
