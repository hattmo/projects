#![feature(array_windows)]
use std::fmt::Debug;

fn main() {
    helper::run("2024", "2", part_1, part_2);
}

fn is_safe(levels: &[usize]) -> bool {
    levels
        .array_windows::<2>()
        .all(|[a, b]| a < b && (b - a) >= 1 && (b - a) <= 3)
        || levels
            .array_windows::<2>()
            .all(|[a, b]| a > b && (a - b) >= 1 && (a - b) <= 3)
}

fn part_1(input: &str) -> impl Debug {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|i| i.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|line| is_safe(line.as_slice()))
        .count()
}

fn part_2(input: &str) -> impl Debug {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|i| i.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|line| {
            if is_safe(line.as_slice()) {
                true
            } else {
                for i in 0..line.len() {
                    let mut line = line.clone();
                    line.remove(i);
                    if is_safe(&line[..]) {
                        return true;
                    }
                }
                false
            }
        })
        .count()
}
