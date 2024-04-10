#![allow(dead_code)]
#![allow(unused_variables)]
use helper::get_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2022", "2", true)?;
    println!("part1 {:?}", part_1(&input));
    println!("part2 {:?}", part_2(&input));
    Ok(())
}
fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(|item| item.split_once(' ').unwrap())
        .map(|round| match round {
            ("A", "X") => 1 + 3,
            ("A", "Y") => 2 + 6,
            ("A", "Z") => 3 + 0,
            ("B", "X") => 1 + 0,
            ("B", "Y") => 2 + 3,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 1 + 6,
            ("C", "Y") => 2 + 0,
            ("C", "Z") => 3 + 3,
            _ => panic!("invalid parse"),
        })
        .sum::<i32>()
}
fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(|item| item.split_once(' ').unwrap())
        .map(|round| match round {
            ("A", "X") => 3,
            ("A", "Y") => 3 + 1,
            ("A", "Z") => 6 + 2,
            ("B", "X") => 1,
            ("B", "Y") => 3 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "X") => 2,
            ("C", "Y") => 3 + 3,
            ("C", "Z") => 6 + 1,
            _ => panic!("invalid parse"),
        })
        .sum::<i32>()
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const TEST_DATA: &str = "A Y
    B X
    C Z
    ";
    #[test]
    fn test() {
        let result = part_1(TEST_DATA);
        assert_eq!(15, result);
    }
    #[test]
    fn test2() {
        let result = part_2(TEST_DATA);
        assert_eq!(12, result);
    }
}
