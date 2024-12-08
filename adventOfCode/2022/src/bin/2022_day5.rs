#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(iter_array_chunks)]
#![feature(map_many_mut)]
use helper::get_input;
use itertools::Itertools;
use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2022", "5", true)?;
    println!("part1 {:?}", part_1(&input));
    println!("part2 {:?}", part_2(&input));
    Ok(())
}
fn part_1(input: &str) -> String {
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    let mut lines = input.lines();

    for line in lines.take_while_ref(|line| line.chars().nth(1).unwrap() != '1') {
        line.chars()
            .chain(" ".chars())
            .array_chunks::<4>()
            .enumerate()
            .for_each(|(i, [_, item, _, _])| {
                if item == ' ' {
                    return;
                }
                let entry = stacks.entry(i + 1).or_default();
                entry.insert(0, item);
            });
    }
    lines.next().unwrap();
    lines.next().unwrap();
    let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in lines {
        let (amount, from, to) = re
            .captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|item| item.unwrap().as_str().trim().parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let [from, to] = stacks.get_many_mut([&from, &to]).unwrap();
        for _ in 0..amount {
            to.push(from.pop().unwrap());
        }
    }
    let mut entries = stacks.into_iter().collect_vec();
    entries.sort_by_key(|(key, _)| *key);
    let result = entries
        .into_iter()
        .filter_map(|(_, val)| val.last().copied())
        .collect::<String>();
    result
}

fn part_2(input: &str) -> String {
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    let mut lines = input.lines();

    for line in lines.take_while_ref(|line| line.chars().nth(1).unwrap() != '1') {
        line.chars()
            .chain(" ".chars())
            .array_chunks::<4>()
            .enumerate()
            .for_each(|(i, [_, item, _, _])| {
                if item == ' ' {
                    return;
                }
                let entry = stacks.entry(i + 1).or_default();
                entry.insert(0, item);
            });
    }
    lines.next().unwrap();
    lines.next().unwrap();
    let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in lines {
        let (amount, from, to) = re
            .captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|item| item.unwrap().as_str().trim().parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let [from, to] = stacks.get_many_mut([&from, &to]).unwrap();
        to.extend(from.drain(from.len() - amount..from.len()));
    }
    let mut entries = stacks.into_iter().collect_vec();
    entries.sort_by_key(|(key, _)| *key);
    let result = entries
        .into_iter()
        .filter_map(|(_, val)| val.last().copied())
        .collect::<String>();
    result
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const TEST_DATA: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
    #[test]
    fn test1() {
        let expected = "CMZ";
        let actual = part_1(TEST_DATA);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test2() {
        let expected = "MCD";
        let actual = part_2(TEST_DATA);
        assert_eq!(expected, actual)
    }
}
