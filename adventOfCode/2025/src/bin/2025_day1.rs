use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

fn main() {
    helper::run("2025", "1", part_1, part_2);
}

enum Dir {
    Left,
    Right,
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dir::Left => write!(f, "L"),
            Dir::Right => write!(f, "R"),
        }
    }
}

impl FromStr for Dir {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Dir::Right),
            "L" => Ok(Dir::Left),
            _ => Err("Failed to parse"),
        }
    }
}

fn part_1(input: &str) -> impl Debug {
    let input = input.lines().map(|line| {
        let (dir, dist) = line.split_at(1);
        let dir: Dir = dir.parse().unwrap();
        let dist: i64 = dist.parse().unwrap();
        (dir, dist)
    });
    let mut on: i64 = 50;
    let mut count = 0;
    for (dir, dist) in input {
        match dir {
            Dir::Left => on -= dist,
            Dir::Right => on += dist,
        }
        while on < 0 {
            on += 100;
        }
        while on > 99 {
            on -= 100
        }
        if on == 0 {
            count += 1;
        }
    }
    count
}
fn part_2(input: &str) -> impl Debug {
    let input = input.lines().map(|line| {
        let (dir, dist) = line.split_at(1);
        let dir: Dir = dir.parse().unwrap();
        let dist: i64 = dist.parse().unwrap();
        (dir, dist)
    });
    let mut on: i64 = 50;
    let mut count = 0;
    for (dir, dist) in input {
        let new = match dir {
            Dir::Left => {
                let mut new = on - dist;
                if on == 0 {
                    count -= 1;
                }
                while new < 0 {
                    count += 1;
                    new += 100;
                }
                if new == 0 {
                    count += 1;
                }
                new
            }
            Dir::Right => {
                let mut new = on + dist;
                while new > 99 {
                    count += 1;
                    new -= 100
                }
                new
            }
        };
        on = new;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_1() {
        let actual = format!("{:?}", part_1(TEST_INPUT));
        assert_eq!(actual, "3");
    }
    #[test]
    fn test_2() {
        let actual = format!("{:?}", part_2(TEST_INPUT));
        assert_eq!(actual, "6");
    }
}
