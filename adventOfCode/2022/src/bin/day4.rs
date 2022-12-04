#![allow(dead_code)]
#![allow(unused_variables)]
use helper::get_input;
use regex::Regex;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2022", "4", true)?;
    println!("part1 {:?}", part_1(&input));
    println!("part2 {:?}", part_2(&input));
    Ok(())
}
fn part_1(input: &str) -> i32 {
    let matcher = Regex::new(r#"^(\d*)-(\d*),(\d*)-(\d*)$"#).unwrap();
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            let matches = matcher.captures(line).unwrap();
            (
                matches.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                matches.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                matches.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                matches.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            )
        })
        .map(|(left_lower, left_upper, right_lower, right_upper)| {
            ((left_lower >= right_lower) && (left_upper <= right_upper))
                || ((left_lower <= right_lower) && (left_upper >= right_upper))
        })
        .filter(|i| *i)
        .count()
        .try_into()
        .unwrap()
}
fn part_2(input: &str) -> i32 {
    let matcher = Regex::new(r#"^(\d*)-(\d*),(\d*)-(\d*)$"#).unwrap();
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            let matches = matcher.captures(line).unwrap();
            (
                matches.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                matches.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                matches.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                matches.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            )
        })
        .map(|(left_lower, left_upper, right_lower, right_upper)| {
            ((left_lower >= right_lower) && (left_lower <= right_upper))
                || ((left_upper >= right_lower) && (left_upper <= right_upper))
                || ((left_lower <= right_lower) && (left_upper >= right_upper))
        })
        .filter(|i| *i)
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const TEST_DATA: &str = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";
    #[test]
    fn test1() {
        let expected = 2;
        let actual = part_1(TEST_DATA);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test2() {
        let expected = 4;
        let actual = part_2(TEST_DATA);
        assert_eq!(expected, actual)
    }
}
