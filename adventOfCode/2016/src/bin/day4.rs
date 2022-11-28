use helper::get_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2016", "4", true)?;
    part_1(&input);
    part_2(&input);
    Ok(())
}
fn part_1(input: &str) {}
fn part_2(input: &str) {}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
