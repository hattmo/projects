use std::{collections::HashMap, error::Error};

use helper::get_input;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2016", "2", true)?;
    // part1(&input);
    part2(&input);
    Ok(())
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct KeyPad {
    val: i32,
}

impl KeyPad {
    pub fn adjust(&mut self, dir: Direction) {
        match dir {
            Direction::Up => {
                if ![1, 2, 3].contains(&self.val) {
                    self.val -= 3;
                };
            }
            Direction::Down => {
                if ![7, 8, 9].contains(&self.val) {
                    self.val += 3;
                };
            }
            Direction::Left => {
                if ![1, 4, 7].contains(&self.val) {
                    self.val -= 1;
                };
            }
            Direction::Right => {
                if ![3, 6, 9].contains(&self.val) {
                    self.val += 1;
                };
            }
        }
    }
}

fn part1(input: &str) {
    let mut keypad = KeyPad { val: 5 };
    let mut total = String::new();
    for line in input.lines() {
        for dir in line.chars() {
            match dir {
                'U' => keypad.adjust(Direction::Up),
                'D' => keypad.adjust(Direction::Down),
                'L' => keypad.adjust(Direction::Left),
                'R' => keypad.adjust(Direction::Right),
                invalid => panic!("Invalid char: {invalid}"),
            }
        }
        total.push_str(&keypad.val.to_string());
    }
    println!("{total}");
}

struct KeyPadEnhanced {
    keys: HashMap<(i32, i32), &'static str>,
    location: (i32, i32),
}

impl Default for KeyPadEnhanced {
    fn default() -> Self {
        let mut keys = HashMap::new();
        keys.insert((2, 0), "D");
        keys.insert((1, 1), "A");
        keys.insert((2, 1), "B");
        keys.insert((3, 1), "C");
        keys.insert((0, 2), "5");
        keys.insert((1, 2), "6");
        keys.insert((2, 2), "7");
        keys.insert((3, 2), "8");
        keys.insert((4, 2), "9");
        keys.insert((1, 3), "2");
        keys.insert((2, 3), "3");
        keys.insert((3, 3), "4");
        keys.insert((2, 4), "1");
        Self {
            keys,
            location: (2, 2),
        }
    }
}
impl KeyPadEnhanced {
    fn adjust(&mut self, dir: Direction) {
        let next_pos = match dir {
            Direction::Up => (self.location.0, self.location.1 + 1),
            Direction::Down => (self.location.0, self.location.1 - 1),
            Direction::Left => (self.location.0 - 1, self.location.1),
            Direction::Right => (self.location.0 + 1, self.location.1),
        };
        if self.keys.keys().any(|item| item == &next_pos) {
            self.location = next_pos;
        }
    }
    fn get_key<'a>(&'a self) -> &'a str {
        return self.keys.get(&self.location).unwrap();
    }
}

fn part2(input: &str) {
    let mut keypad = KeyPadEnhanced::default();
    let mut total = String::new();
    for line in input.lines() {
        for dir in line.chars() {
            match dir {
                'U' => keypad.adjust(Direction::Up),
                'D' => keypad.adjust(Direction::Down),
                'L' => keypad.adjust(Direction::Left),
                'R' => keypad.adjust(Direction::Right),
                invalid => panic!("Invalid char: {invalid}"),
            }
        }
        total.push_str(&keypad.get_key());
    }
    println!("{total}");
}
