#![allow(dead_code)]
#![allow(unused_variables)]
use helper::get_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2022", "1", true)?;
    println!("part1 {:?}", part_1(&input));
    println!("part2 {:?}", part_2(&input));
    Ok(())
}
fn part_1(input: &str) -> i32 {
    0
}
fn part_2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const TEST_DATA: &str = "TEST_DATA";
    #[test]
    fn test1() {
        let expected = 0;
        let actual = part_1(TEST_DATA);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test2() {
        let expected = 0;
        let actual = part_2(TEST_DATA);
        assert_eq!(expected, actual)
    }
}
