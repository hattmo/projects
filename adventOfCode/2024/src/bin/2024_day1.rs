use std::fmt::Debug;

fn main() {
    helper::run("2024", "1", part_1, part_2);
}

fn part_1(input: &str) -> impl Debug {
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
fn part_2(input: &str) -> impl Debug {
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
