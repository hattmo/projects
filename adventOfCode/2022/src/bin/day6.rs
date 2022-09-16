#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(array_windows)]
use helper::get_input;
use itertools::Itertools;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2022", "6", true)?;
    println!("part1 {:?}", part_1(&input));
    println!("part2 {:?}", part_2(&input));
    Ok(())
}
fn part_1(input: &str) -> usize {
    input
        .chars()
        .into_iter()
        .tuple_windows()
        .enumerate()
        .find_map(|(index, (first, second, third, fourth))| {
            if [first, second, third, fourth].into_iter().all_unique() {
                println!("{} {} {} {}", first, second, third, fourth);
                Some(index + 4)
            } else {
                None
            }
        })
        .unwrap()
}
fn part_2(input: &str) -> usize {
    input
        .chars()
        .into_iter()
        .collect_vec()
        .array_windows::<14>()
        .enumerate()
        .find_map(|(index, windows)| {
            if windows.into_iter().all_unique() {
                Some(index + 14)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const TEST_DATA: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TEST_DATA2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    #[test]
    fn test1() {
        let expected = 7;
        let actual = part_1(TEST_DATA);
        assert_eq!(expected, actual);

        let expected = 5;
        let actual = part_1(TEST_DATA2);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test2() {
        let expected = 19;
        let actual = part_2(TEST_DATA);
        assert_eq!(expected, actual);

        let expected = 23;
        let actual = part_2(TEST_DATA2);
        assert_eq!(expected, actual);
    }
}
