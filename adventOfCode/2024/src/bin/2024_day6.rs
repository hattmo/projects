use std::{cell::RefCell, collections::HashMap, error::Error, fmt::Debug, ops::Add, rc::Rc};

fn main() -> Result<(), Box<dyn Error>> {
    let input = helper::get_input("2024", "6", true)?;
    let res = part1(&input);
    println!("part1: {res:?}");
    let res = part2(&input);
    println!("part2: {res:?}");
    Ok(())
}
#[derive(Debug, Clone)]
enum Space {
    Blank,
    Block,
    Visited(Vec<Dir>),
}

impl Space {
    fn is_visited(&self) -> bool {
        match self {
            Space::Visited(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn_right(&mut self) {
        let new = match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        };
        *self = new;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Add<Dir> for Coord {
    type Output = Coord;

    fn add(self, rhs: Dir) -> Self::Output {
        let Coord { x, y } = self;
        match rhs {
            Dir::Up => Coord { x, y: y - 1 },
            Dir::Down => Coord { x, y: y + 1 },
            Dir::Left => Coord { x: x - 1, y },
            Dir::Right => Coord { x: x + 1, y },
        }
    }
}

fn parse_map(data: &str) -> (Coord, HashMap<Coord, Space>) {
    let coord = Rc::new(RefCell::new(None));
    let out: HashMap<_, _> = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let coord = coord.clone();
            line.chars().enumerate().map(move |(col, item)| {
                let space = match item {
                    '.' => Space::Blank,
                    '#' => Space::Block,
                    '^' => {
                        let mut coord = coord.borrow_mut();
                        *coord = Some(Coord { x: col, y: row });
                        Space::Visited(vec![Dir::Up])
                    }
                    _ => panic!("unexpected input"),
                };
                (Coord { x: col, y: row }, space)
            })
        })
        .flatten()
        .collect();
    let coord = coord.take().unwrap();
    (coord, out)
}

fn resolve_board(mut coord: Coord, board: &mut HashMap<Coord, Space>) -> bool {
    let mut dir = Dir::Up;
    loop {
        let dst = coord + dir;
        let dst_node = board.get_mut(&dst);
        match dst_node {
            Some(dst_node @ Space::Blank) => {
                *dst_node = Space::Visited(vec![dir]);
                coord = dst;
            }
            Some(Space::Block) => dir.turn_right(),
            Some(Space::Visited(dirs)) => {
                if dirs.contains(&dir) {
                    return true;
                }
                dirs.push(dir);
                coord = dst
            }
            None => return false,
        }
    }
}

fn part1(input: &str) -> impl Debug + use<'_> {
    let (start_coord, mut board) = parse_map(input);
    resolve_board(start_coord, &mut board);
    board.values().filter(|space| space.is_visited()).count()
}

fn part2(input: &str) -> impl Debug + use<'_> {
    let (start_coord, blank_board) = parse_map(input);
    let mut orig_board = blank_board.clone();
    resolve_board(start_coord, &mut orig_board);
    orig_board
        .into_iter()
        .filter(|(_, val)| val.is_visited())
        .filter(move |(key, _)| *key != start_coord)
        .map(|(key, _)| key)
        .filter(|potential_block| {
            let mut test_board = blank_board.clone();
            *test_board.get_mut(potential_block).unwrap() = Space::Block;
            resolve_board(start_coord, &mut test_board)
        })
        .count()
}
