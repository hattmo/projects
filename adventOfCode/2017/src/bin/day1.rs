#![allow(dead_code)]
#![allow(unused_variables)]
use helper::get_input;
use itertools::Itertools;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2017", "1", true)?;
    println!("part1 {:?}", part_1(&input));
    println!("part2 {:?}", part_2(&input));
    Ok(())
}
fn part_1(input: &str) -> i32 {
    input
        .trim()
        .chars()
        .collect::<Vec<_>>()
        .into_iter()
        .circular_tuple_windows()
        .map(|(this, next)| {
            if this == next {
                return this.to_digit(10).unwrap().try_into().unwrap();
            } else {
                0i32
            }
        })
        .sum::<i32>()
        .try_into()
        .unwrap()
}
fn part_2(input: &str) -> i32 {
    let chars = input.trim().chars().collect_vec();
    let left = &chars[..chars.len() / 2];
    let right = &chars[chars.len() / 2..chars.len()];
    left.into_iter()
        .zip(right.into_iter())
        .map(|(left, right)| {
            if left == right {
                return left.to_digit(10).unwrap().try_into().unwrap();
            } else {
                0i32
            }
        })
        .sum::<i32>()
        * 2i32
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const TEST_DATA: &str = "91212129";
    #[test]
    fn test1() {
        let expected = 9;
        let actual = part_1(TEST_DATA);
        assert_eq!(expected, actual)
    }

    const TEST_DATA2: &str = "12131415";
    #[test]
    fn test2() {
        let expected = 4;
        let actual = part_2(TEST_DATA2);
        assert_eq!(expected, actual)
    }
}
