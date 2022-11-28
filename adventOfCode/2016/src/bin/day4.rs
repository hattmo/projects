#![feature(string_remove_matches)]
use helper::get_input;
use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2016", "4", true)?;
    let solution = part_1(&input);
    println!("part1 {solution}");
    part_2(&input);
    Ok(())
}

fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (encrypted_name, rest) = line.rsplit_once("-").expect("Bad Parse");
            let (sector_id, check_sum) = rest.split_once("[").expect("Bad Parse");

            let check_sum = &check_sum.trim().trim_matches(']');
            let mut encrypted_name = encrypted_name.to_string();
            let sector_id: i32 = sector_id.parse().expect("Bad Parse");

            encrypted_name.remove_matches("-");
            let mut char_count: HashMap<char, usize> = HashMap::new();
            encrypted_name.chars().for_each(|item| {
                *char_count.entry(item).or_insert(0) += 1;
            });
            let mut counts = char_count.iter().collect::<Vec<_>>();
            counts.sort_by(|a, b| {
                let ord = b.1.cmp(a.1);
                if ord.is_eq() {
                    return a.0.cmp(b.0);
                } else {
                    return ord;
                }
            });
            let is_valid = counts
                .iter()
                .take(5)
                .map(|i| i.0)
                .filter(|i| check_sum.contains(**i))
                .count()
                == 5;
            return (is_valid, sector_id);
        })
        .filter(|(valid, _)| *valid)
        .map(|i| i.1)
        .reduce(|acc, next| acc + next)
        .unwrap_or(0)
}

fn part_2(input: &str) {}

#[cfg(test)]
mod test {
    use crate::part_1;

    #[test]
    fn test1() {
        let test_data = "aaaaa-bbb-z-y-x-123[abxyz]";
        let total = part_1(test_data);
        assert_eq!(123, total);
    }
    #[test]
    fn test2() {
        let test_data = "a-b-c-d-e-f-g-h-987[abcde]";
        let total = part_1(test_data);
        assert_eq!(987, total);
    }
    #[test]
    fn test3() {
        let test_data = "not-a-real-room-404[oarel]";
        let total = part_1(test_data);
        assert_eq!(404, total);
    }
    #[test]
    fn test4() {
        let test_data = "totally-real-room-200[decoy]";
        let total = part_1(test_data);
        assert_eq!(0, total);
    }
}
