#![allow(dead_code)]
#![allow(unused_variables)]
use helper::get_input;
use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2016", "6", true)?;
    println!("part1 {}", part_1(&input));
    println!("part1 {}", part_2(&input));
    Ok(())
}
fn part_1(input: &str) -> String {
    let map = input
        .lines()
        .map(str::trim)
        .map(|line| line.chars().collect())
        .fold(
            HashMap::new(),
            |mut map: HashMap<(usize, char), usize>, items: Vec<char>| {
                for key in items.into_iter().enumerate() {
                    *map.entry(key).or_default() += 1;
                }
                map
            },
        );
    let mut out = String::new();
    for i in 0usize..8 {
        let mut letters_in_col = map
            .iter()
            .filter(|((index, _), _)| *index == i)
            .collect::<Vec<_>>();
        letters_in_col.sort_by_key(|(_, count)| count.clone());
        let ((_, foo), _) = letters_in_col.last().expect("Failed to find first");
        out.push(*foo);
    }
    out
}
fn part_2(input: &str) -> String {
    let map = input
        .lines()
        .map(str::trim)
        .map(|line| line.chars().collect())
        .fold(
            HashMap::new(),
            |mut map: HashMap<(usize, char), usize>, items: Vec<char>| {
                for key in items.into_iter().enumerate() {
                    *map.entry(key).or_default() += 1;
                }
                map
            },
        );
    let mut out = String::new();
    for i in 0usize..8 {
        let mut letters_in_col = map
            .iter()
            .filter(|((index, _), _)| *index == i)
            .collect::<Vec<_>>();
        letters_in_col.sort_by_key(|(_, count)| count.clone());
        let ((_, foo), _) = letters_in_col.first().expect("Failed to find first");
        out.push(*foo);
    }
    out
}

#[cfg(test)]
mod test {
    use crate::part_1;

    #[test]
    fn test1() {
        let test_data = "eedadn
        drvtee
        eandsr
        raavrd
        atevrs
        tsrnev
        sdttsa
        rasrtv
        nssdts
        ntnada
        svetve
        tesnvt
        vntsnd
        vrdear
        dvrsen
        enarar";

        let result = part_1(test_data);
        assert_eq!(result, "easter".to_owned())
    }
}
