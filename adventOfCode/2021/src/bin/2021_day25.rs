use std::{
    cmp::max,
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    io::stdin,
    ops::RangeInclusive,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = helper::get_input("2021", "25", true)?;
    let res = part1(&input);
    println!("part1: {res:?}");
    let res = part2(&input);
    println!("part2: {res:?}");
    Ok(())
}
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point(usize, usize);

impl Point {
    fn down(&self, height: &RangeInclusive<usize>) -> Point {
        let &Point(x, mut y) = self;
        y += 1;
        if height.contains(&y) {
            Point(x, y)
        } else {
            Point(x, 0)
        }
    }
    fn right(&self, width: &RangeInclusive<usize>) -> Point {
        let &Point(mut x, y) = self;
        x += 1;
        if width.contains(&x) {
            Point(x, y)
        } else {
            Point(0, y)
        }
    }
}

enum Cucumber {
    East,
    South,
}

impl Cucumber {
    fn is_south(&self) -> bool {
        match self {
            Cucumber::East => false,
            Cucumber::South => true,
        }
    }
    fn is_east(&self) -> bool {
        !self.is_south()
    }
}

#[allow(dead_code)]
struct MapPrinter<'a>(
    &'a HashMap<Point, Cucumber>,
    &'a RangeInclusive<usize>,
    &'a RangeInclusive<usize>,
);

impl<'a> Display for MapPrinter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &MapPrinter(map, width, height) = self;
        let mut out = String::new();
        for y in height.to_owned() {
            for x in width.to_owned() {
                match map.get(&Point(x, y)) {
                    Some(Cucumber::East) => out += ">",
                    Some(Cucumber::South) => out += "v",
                    None => out += ".",
                }
            }
            out += "\n"
        }
        write!(f, "{out}")
    }
}

fn parse_map(
    input: &str,
) -> (
    (RangeInclusive<usize>, RangeInclusive<usize>),
    HashMap<Point, Cucumber>,
) {
    let mut out = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (row, line) in input.lines().enumerate() {
        height = max(height, row);
        for (col, c) in line.chars().enumerate() {
            width = max(width, col);
            match c {
                '>' => {
                    out.insert(Point(col, row), Cucumber::East);
                }
                'v' => {
                    out.insert(Point(col, row), Cucumber::South);
                }
                _ => {}
            };
        }
    }

    let foo = ((0..=width, 0..=height), out);
    foo
}

fn part1(input: &str) -> impl Debug + use<'_> {
    let ((width, height), mut map) = parse_map(input);
    let mut removes = Vec::new();
    let mut adds = Vec::new();
    let mut did_move = true;
    let mut step = 0;
    while did_move {
        did_move = false;
        step += 1;
        if cfg!(test) {
            let printer = MapPrinter(&map, &width, &height);
            println!("{printer}");
            let _ = stdin().read_line(&mut String::new());
        }
        for (coord, _) in map.iter().filter(|(_, v)| v.is_east()) {
            let new = coord.right(&width);
            if !map.contains_key(&new) {
                removes.push(coord.clone());
                adds.push(new);
            };
        }
        if !adds.is_empty() {
            did_move = true;
        }
        for add in adds.iter() {
            map.insert(*add, Cucumber::East);
        }
        for remove in removes.iter() {
            map.remove(&remove);
        }

        removes.clear();
        adds.clear();

        for (coord, _) in map.iter().filter(|(_, v)| v.is_south()) {
            let new = coord.down(&height);
            if !map.contains_key(&new) {
                removes.push(coord.clone());
                adds.push(new);
            };
        }
        if !adds.is_empty() {
            did_move = true;
        }
        for add in adds.iter() {
            map.insert(*add, Cucumber::South);
        }
        for remove in removes.iter() {
            map.remove(&remove);
        }
        removes.clear();
        adds.clear();
    }
    step
}
fn part2(_input: &str) -> impl Debug + use<'_> {
    "todo"
}
#[cfg(test)]
mod test {
    use crate::part1;
    #[test]
    fn test_part1() {
        let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
        let res = part1(input);
        println!("{res:?}");
    }
}
