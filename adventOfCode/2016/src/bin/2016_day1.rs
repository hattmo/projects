use std::error::Error;

use helper::get_input;

fn main() -> Result<(), Box<dyn Error>> {
    part_1()?;
    part_2()?;
    Ok(())
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part_1() -> Result<(), Box<dyn Error>> {
    let input = get_input("2016", "1", true)?;
    let mut x_distance = 0;
    let mut y_distance = 0;
    let mut direction = Direction::Up;
    for item in input.split(",").map(|i| i.trim()) {
        match (&item[..1], &item[1..].parse::<i32>().unwrap()) {
            ("R", distance) => match direction {
                Direction::Up => {
                    direction = Direction::Right;
                    x_distance += distance;
                }
                Direction::Down => {
                    direction = Direction::Left;
                    x_distance -= distance;
                }
                Direction::Left => {
                    direction = Direction::Up;
                    y_distance += distance;
                }
                Direction::Right => {
                    direction = Direction::Down;
                    y_distance -= distance;
                }
            },
            ("L", distance) => match direction {
                Direction::Up => {
                    direction = Direction::Left;
                    x_distance -= distance;
                }
                Direction::Down => {
                    direction = Direction::Right;
                    x_distance += distance;
                }
                Direction::Left => {
                    direction = Direction::Down;
                    y_distance -= distance;
                }
                Direction::Right => {
                    direction = Direction::Up;
                    y_distance += distance;
                }
            },
            (val, dis) => {
                println!("{val} {dis}");
                panic!("Invalid input")
            }
        };
    }
    let total = x_distance.abs() + y_distance.abs();
    println!("part1: {total}");
    Ok(())
}

fn part_2() -> Result<(), Box<dyn Error>> {
    let input = get_input("2016", "1", true)?;
    let mut visited = Vec::<(i32, i32)>::new();
    let mut current_loc = (0, 0);
    visited.push(current_loc.clone());
    let mut cur_dir = Direction::Up;
    'outer: for item in input.split(",").map(|i| i.trim()) {
        let direction = &item[..1];
        let distance = &item[1..].parse::<i32>().unwrap();
        if direction == "R" {
            match cur_dir {
                Direction::Up => cur_dir = Direction::Right,
                Direction::Down => cur_dir = Direction::Left,
                Direction::Left => cur_dir = Direction::Up,
                Direction::Right => cur_dir = Direction::Down,
            }
        } else if direction == "L" {
            match cur_dir {
                Direction::Up => cur_dir = Direction::Left,
                Direction::Down => cur_dir = Direction::Right,
                Direction::Left => cur_dir = Direction::Down,
                Direction::Right => cur_dir = Direction::Up,
            }
        }

        match cur_dir {
            Direction::Up => {
                for _ in 0..*distance {
                    current_loc = (current_loc.0, current_loc.1 + 1);
                    if visited.contains(&current_loc) {
                        break 'outer;
                    } else {
                        visited.push(current_loc.clone());
                    };
                }
            }
            Direction::Down => {
                for _ in 0..*distance {
                    current_loc = (current_loc.0, current_loc.1 - 1);
                    if visited.contains(&current_loc) {
                        break 'outer;
                    } else {
                        visited.push(current_loc.clone());
                    };
                }
            }
            Direction::Left => {
                for _ in 0..*distance {
                    current_loc = (current_loc.0 - 1, current_loc.1);
                    if visited.contains(&current_loc) {
                        break 'outer;
                    } else {
                        visited.push(current_loc.clone());
                    };
                }
            }
            Direction::Right => {
                for _ in 0..*distance {
                    current_loc = (current_loc.0 + 1, current_loc.1);
                    if visited.contains(&current_loc) {
                        break 'outer;
                    } else {
                        visited.push(current_loc.clone());
                    };
                }
            }
        }
    }
    let result = current_loc.0.abs() + current_loc.1.abs();
    println!("part2: {result}");
    Ok(())
}
