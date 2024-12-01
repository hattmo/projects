use std::{error::Error, fmt::Display};

fn main() -> Result<(), Box<dyn Error>> {
    let input = helper::get_input("2024", "1", true)?;
    let res = part1(&input);
    println!("part1: {res}");
    let res = part2(&input);
    println!("part2: {res}");
    Ok(())
}

fn part1(input: &str) -> impl Display {
    let (mut left, mut right): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(left, right)| {
            (
                left.trim().parse::<u64>().unwrap(),
                right.trim().parse::<u64>().unwrap(),
            )
        })
        .unzip();
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right.into_iter())
        .map(|(left, right)| left.abs_diff(right))
        .sum::<u64>()
}
fn part2(input: &str) -> impl Display {
    let (left, right): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(left, right)| {
            (
                left.trim().parse::<u64>().unwrap(),
                right.trim().parse::<u64>().unwrap(),
            )
        })
        .unzip();
    let mut total = 0;
    for item in left.into_iter() {
        let count = right.iter().filter(|i| **i == item).count() as u64;
        total += count * item;
    }
    total
}
