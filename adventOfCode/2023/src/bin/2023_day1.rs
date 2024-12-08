use std::{error::Error, fmt::Debug};

fn main() -> Result<(), Box<dyn Error>> {
    let input = helper::get_input("2024", "1", true)?;
    let res = part1(&input);
    println!("part1: {res:?}");
    let res = part2(&input);
    println!("part2: {res:?}");
    Ok(())
}

fn part1(_input: &str) -> impl Debug + use<'_> {
    "todo"
}
fn part2(_input: &str) -> impl Debug + use<'_> {
    "todo"
}
