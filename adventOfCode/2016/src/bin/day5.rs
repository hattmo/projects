#![allow(dead_code)]
#![allow(unused_variables)]
use helper::get_input;
use std::{error::Error, process::exit};

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2016", "5", true)?;
    let input = input.trim();
    println!("part1 {}", part_1(input));
    part_2(&input);
    Ok(())
}

fn compute_password(door_id: &str) -> String {
    let mut index = 0u64;
    let mut out = String::new();
    loop {
        let test = format!("{door_id}{index}");
        let result = format!("{:x}", md5::compute(test));
        if &result[0..5] == "00000" {
            out = format!("{}{}", out, &result[5..6]);
            if out.len() == 8 {
                return out;
            }
        }
        index += 1;
    }
}

fn part_1(input: &str) -> String {
    return compute_password(input);
}
fn part_2(input: &str) {}

#[cfg(test)]
mod test {
    use crate::compute_password;

    #[test]
    fn test2() {
        let door_id = "abc";
        let password = compute_password(door_id);
        assert_eq!("18f47a30", password)
    }
}
