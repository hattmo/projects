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
fn part_1(input: &str) {}
fn part_2(input: &str) {}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
