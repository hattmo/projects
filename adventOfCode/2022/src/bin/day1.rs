#![allow(dead_code)]
#![allow(unused_variables)]
use helper::get_input;
use itertools::Itertools;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = get_input("2022", "1", true)?;
    input.push('\n');
    println!("part1 {:?}", part_1(&input));
    println!("part2 {:?}", part_2(&input));
    Ok(())
}
fn part_1(input: &str) -> i32 {
    input
        .lines()
        .into_iter()
        .map(str::trim)
        .group_by(|item| *item != "")
        .into_iter()
        .filter(|(group, val)| *group)
        .map(|(_, group)| group.map(|i| i.parse::<i32>().unwrap()).sum::<i32>())
        .sorted()
        .rev()
        .next()
        .unwrap()
}
fn part_2(input: &str) -> i32 {
    input
        .lines()
        .into_iter()
        .map(str::trim)
        .group_by(|item| *item != "")
        .into_iter()
        .filter(|(group, val)| *group)
        .map(|(_, group)| group.map(|i| i.parse::<i32>().unwrap()).sum::<i32>())
        .sorted()
        .rev()
        .take(3)
        .sum()

    // .filter(|(group, _)| group)
    // // .fold((Vec::<i32>::new(), 0), |(mut totals, current), item| {
    // //     if item == "" {
    // //         totals.push(current);
    // //         (totals, 0)
    // //     } else {
    // //         (totals, current + item.parse::<i32>().unwrap())
    // //     }
    // // })
    // // .0
    // .into_iter()
    // .sorted()
    // .rev()
    // .take(3)
    // .sum()
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};
    const TEST_DATA: &str = "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000
    ";
    #[test]
    fn test() {
        let result = part_1(TEST_DATA);
        assert_eq!(result, 24000)
    }
    #[test]
    fn test2() {
        let result = part_2(TEST_DATA);
        assert_eq!(result, 45000)
    }
}
