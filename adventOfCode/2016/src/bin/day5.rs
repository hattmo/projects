#![allow(dead_code)]
#![allow(unused_variables)]
use helper::get_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2016", "5", true)?;
    let input = input.trim();
    println!("part1 {}", part_1(input));
    println!("part2 {}", part_2(input));
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

fn compute_password2(door_id: &str) -> String {
    let mut index = 0u64;
    let mut out = vec![Option::<String>::None; 8];

    loop {
        let test = format!("{door_id}{index}");
        let result = format!("{:x}", md5::compute(test));
        if &result[0..5] == "00000" {
            if let Ok(placement) = result[5..6].parse::<usize>() {
                if placement < out.len() && out[placement].is_none() {
                    out[placement] = Some(result[6..7].to_owned());
                    if out.iter().all(Option::is_some) {
                        break;
                    }
                }
            }
        }
        index += 1;
    }

    out.into_iter().map(Option::unwrap).collect::<String>()
}

fn part_1(input: &str) -> String {
    compute_password(input)
}
fn part_2(input: &str) -> String {
    compute_password2(input)
}

#[cfg(test)]
mod test {
    use crate::{compute_password, compute_password2};

    #[test]
    fn test1() {
        let door_id = "abc";
        let password = compute_password(door_id);
        assert_eq!("18f47a30", password)
    }
    #[test]
    fn test2() {
        let door_id = "abc";
        let password = compute_password2(door_id);
        assert_eq!("05ace8e3", password)
    }
}
