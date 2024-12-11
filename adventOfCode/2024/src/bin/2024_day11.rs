use std::{collections::HashMap, fmt::Debug};

fn main() {
    helper::run("2024", "11", part_1, part_2);
}

fn digits(mut num: u128) -> u128 {
    let mut dig = 0;
    while num > 0 {
        num /= 10;
        dig += 1;
    }
    dig
}

fn pow(mut num: u128, pow: u128) -> u128 {
    let mul = num;
    for _ in 0..(pow - 1) {
        num *= mul;
    }
    num
}

fn count_stones(input: &str, steps: usize) -> u128 {
    let mut stones = input
        .split(" ")
        .map(|i| i.trim())
        .filter(|i| !i.is_empty())
        .map(|i| (i.parse::<u128>().unwrap(), 1))
        .collect::<HashMap<u128, u128>>();
    let mut next = HashMap::new();
    for _ in 0..steps {
        for (stone, num) in stones.iter() {
            if *stone == 0 {
                let entry = next.entry(1).or_insert(0);
                *entry += num;
            } else {
                let dig = digits(*stone);
                if dig % 2 == 0 {
                    let entry = next.entry(*stone / (pow(10, dig / 2))).or_insert(0);
                    *entry += num;
                    let entry = next.entry(*stone % (pow(10, dig / 2))).or_insert(0);
                    *entry += num;
                } else {
                    let entry = next.entry(*stone * 2024).or_insert(0);
                    *entry += num;
                }
            }
        }
        stones = next;
        next = HashMap::new();
    }
    stones.values().sum()
}

fn part_1(input: &str) -> impl Debug {
    count_stones(input, 25)
}

fn part_2(input: &str) -> impl Debug {
    count_stones(input, 75)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "125 17";

    #[test]
    fn test_1() {
        let actual = format!("{:?}", part_1(TEST_INPUT));
        assert_eq!(actual, "55312");
    }
}
