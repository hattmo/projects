#![feature(slice_split_once)]

use std::{collections::HashSet, error::Error, fmt::Debug, usize};

fn main() -> Result<(), Box<dyn Error>> {
    let input = helper::get_input("2024", "5", true)?;
    let res = part1(&input);
    println!("part1: {res:?}");
    let res = part2(&input);
    println!("part2: {res:?}");
    Ok(())
}

fn part1(input: &str) -> impl Debug + use<'_> {
    let lines: Vec<_> = input.lines().collect();
    let (rules, updates) = lines.split_once(|line| line.is_empty()).unwrap();

    let rules: HashSet<_> = rules
        .into_iter()
        .map(|line| line.split_once("|").unwrap())
        .map(|(before, after)| {
            (
                before.parse::<usize>().unwrap(),
                after.parse::<usize>().unwrap(),
            )
        })
        .collect();
    updates
        .into_iter()
        .map(|line| {
            line.split(",")
                .map(|item| item.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|line| {
            line.pairs()
                .all(|Pair { left, right, .. }| rules.contains(&(*left, *right)))
        })
        .map(|good_lines| {
            let len = good_lines.len();
            let half = len / 2;
            good_lines[half]
        })
        .sum::<usize>()
}
struct Pair<'a, T> {
    left: &'a T,
    right: &'a T,
    left_i: usize,
    right_i: usize,
}
struct PairIterator<'a, T> {
    left: usize,
    right: usize,
    data: &'a Vec<T>,
}

trait Pairs<T> {
    fn pairs(&self) -> PairIterator<T>;
}

impl<T> Pairs<T> for Vec<T> {
    fn pairs(&self) -> PairIterator<T> {
        PairIterator {
            data: self,
            left: 0,
            right: 0,
        }
    }
}

impl<'a, T> Iterator for PairIterator<'a, T> {
    type Item = Pair<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.right += 1;
        if self.right >= self.data.len() {
            self.left += 1;
            self.right = self.left + 1;
        }
        if self.right >= self.data.len() {
            return None;
        }
        let left = &self.data[self.left];
        let right = &self.data[self.right];
        Some(Pair {
            left,
            right,
            left_i: self.left,
            right_i: self.right,
        })
    }
}

fn part2(input: &str) -> impl Debug + use<'_> {
    let lines: Vec<_> = input.lines().collect();
    let (rules, updates) = lines.split_once(|line| line.is_empty()).unwrap();

    let rules: HashSet<_> = rules
        .into_iter()
        .map(|line| line.split_once("|").unwrap())
        .map(|(before, after)| {
            (
                before.parse::<usize>().unwrap(),
                after.parse::<usize>().unwrap(),
            )
        })
        .collect();
    let mut incorrect = updates
        .into_iter()
        .map(|line| {
            line.split(",")
                .map(|item| item.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|line| {
            line.pairs()
                .any(|Pair { left, right, .. }| !rules.contains(&(*left, *right)))
        })
        .collect::<Vec<_>>();
    for line in incorrect.iter_mut() {
        loop {
            let mut to_swap = None;
            for Pair {
                left,
                right,
                left_i,
                right_i,
            } in line.pairs()
            {
                if !rules.contains(&(*left, *right)) {
                    to_swap = Some((left_i, right_i));
                    break;
                }
            }
            match to_swap {
                Some((left, right)) => line.swap(left, right),
                None => break,
            }
        }
    }
    incorrect
        .into_iter()
        .map(|line| {
            let len = line.len();
            let mid = len / 2;
            line[mid]
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn test() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let actual = part2(&input);
        println!("{actual:?}");
        assert!(false)
    }
}
