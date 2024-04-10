#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use helper::get_input;
use itertools::Itertools;
use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2022", "8", true)?;
    println!("part1 {:?}", part_1(&input));
    println!("part2 {:?}", part_2(&input));
    Ok(())
}
fn part_1(input: &str) -> usize {
    let mut trees = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    input
        .lines()
        .map(str::trim)
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                trees.insert((x, y), (c.to_digit(10).unwrap().try_into().unwrap(), false));
                max_x = x;
            });
            max_y = y;
        });
    for x in 0..=max_x {
        let mut min_height = -1;
        for y in 0..=max_y {
            let (this_height, visible) = trees.get_mut(&(x, y)).unwrap();
            if *this_height > min_height {
                *visible = true;
                min_height = *this_height;
            }
        }
    }
    for x in 0..=max_x {
        let mut min_height = -1;
        for y in (0..=max_y).rev() {
            let (this_height, visible) = trees.get_mut(&(x, y)).unwrap();
            if *this_height > min_height {
                *visible = true;
                min_height = *this_height;
            }
        }
    }
    for y in 0..=max_y {
        let mut min_height = -1;
        for x in 0..=max_x {
            let (this_height, visible) = trees.get_mut(&(x, y)).unwrap();
            if *this_height > min_height {
                *visible = true;
                min_height = *this_height;
            }
        }
    }
    for y in 0..=max_y {
        let mut min_height = -1;
        for x in (0..=max_x).rev() {
            let (this_height, visible) = trees.get_mut(&(x, y)).unwrap();
            if *this_height > min_height {
                *visible = true;
                min_height = *this_height;
            }
        }
    }
    trees
        .into_iter()
        .filter(|(_, (_, visible))| *visible)
        .count()
}
fn part_2(input: &str) -> i32 {
    let mut trees = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    input
        .lines()
        .map(str::trim)
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                trees.insert((x, y), c.to_digit(10).unwrap());
                max_x = x;
            });
            max_y = y;
        });
    trees
        .iter()
        .map(|((this_x, this_y), this_height)| {
            let mut score = 1;
            // println!("on: {} {} ({})", this_x, this_y, this_height);
            {
                let mut count = 0;
                for x in (*this_x + 1)..=(max_x) {
                    count += 1;
                    if trees.get(&(x, *this_y)).unwrap() >= this_height {
                        break;
                    }
                }
                score *= count;
            }
            {
                let mut count = 0;
                if this_x > &0 {
                    for x in (0..=(this_x - 1)).rev() {
                        count += 1;
                        if trees.get(&(x, *this_y)).unwrap() >= this_height {
                            break;
                        }
                    }
                }
                score *= count;
            }
            {
                let mut count = 0;
                for y in (*this_y + 1)..=(max_y) {
                    count += 1;
                    if trees.get(&(*this_x, y)).unwrap() >= this_height {
                        break;
                    }
                }
                score *= count;
            }
            {
                let mut count = 0;
                if this_y > &0 {
                    for y in (0..=(this_y - 1)).rev() {
                        count += 1;
                        if trees.get(&(*this_x, y)).unwrap() >= this_height {
                            break;
                        }
                    }
                }
                score *= count;
            }
            // println!("{}", score);
            score
        })
        .max()
        .unwrap()
}

fn pause() {
    let std_in = std::io::stdin();
    let mut input = String::new();
    std_in.read_line(&mut input).unwrap();
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};
    const TEST_DATA: &str = r#"30373
    25512
    65332
    33549
    35390"#;
    #[test]
    fn test1() {
        let expected = 21;
        let actual = part_1(TEST_DATA);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let expected = 8;
        let actual = part_2(TEST_DATA);
        assert_eq!(expected, actual);
    }
}
