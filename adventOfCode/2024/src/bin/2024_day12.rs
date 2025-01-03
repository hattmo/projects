use std::{
    collections::HashMap,
    convert::identity,
    fmt::Debug,
    ops::{Add, AddAssign},
};

fn main() {
    helper::run("2024", "12", part_1, part_2);
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn neighbor(&self, side: &Side) -> Point {
        let &Self { x, y } = self;
        match side {
            Side::Top => Point { x, y: y + 1 },
            Side::Bottom => Point { x, y: y - 1 },
            Side::Left => Point { x: x - 1, y },
            Side::Right => Point { x: x + 1, y },
        }
    }
    fn neighbors(&self) -> [(Side, Point); 4] {
        [
            (
                Side::Right,
                Point {
                    x: self.x + 1,
                    y: self.y,
                },
            ),
            (
                Side::Left,
                Point {
                    x: self.x - 1,
                    y: self.y,
                },
            ),
            (
                Side::Top,
                Point {
                    x: self.x,
                    y: self.y + 1,
                },
            ),
            (
                Side::Bottom,
                Point {
                    x: self.x,
                    y: self.y - 1,
                },
            ),
        ]
    }
}

#[derive(Clone, Copy, Debug)]
struct Region {
    ty: char,
    visited: bool,
    left_b: bool,
    right_b: bool,
    top_b: bool,
    bottom_b: bool,
}

impl Region {
    fn has_border(&self, side: Side) -> bool {
        match side {
            Side::Top => self.top_b,
            Side::Bottom => self.bottom_b,
            Side::Left => self.left_b,
            Side::Right => self.right_b,
        }
    }
    fn set_border(&mut self, side: Side) {
        match side {
            Side::Top => self.top_b = true,
            Side::Bottom => self.bottom_b = true,
            Side::Left => self.left_b = true,
            Side::Right => self.right_b = true,
        };
    }
}

#[derive(Clone, Copy, Debug)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, Default, Debug)]
struct Score {
    borders: usize,
    nodes: usize,
}

impl Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            borders: self.borders + rhs.borders,
            nodes: self.nodes + rhs.nodes,
        }
    }
}

impl AddAssign for Score {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

fn parse_map(input: &str) -> HashMap<Point, Region> {
    let lines = input.lines();
    lines
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    Point { x: x + 1, y: y + 1 },
                    Region {
                        ty: c,
                        visited: false,
                        top_b: false,
                        bottom_b: false,
                        left_b: false,
                        right_b: false,
                    },
                )
            })
        })
        .flatten()
        .collect()
}

fn find_next(map: &HashMap<Point, Region>) -> Option<(Point, Region)> {
    map.iter()
        .filter(|(_, r)| !r.visited)
        .map(|(p, r)| (p.clone(), r.clone()))
        .next()
}

fn visit_region((p, r): (Point, Region), m: &mut HashMap<Point, Region>) -> Score {
    m.get_mut(&p).unwrap().visited = true;
    let mut score = Score::default();
    for (_, n_p) in p.neighbors() {
        if let Some(&n_r) = m.get(&n_p) {
            if n_r.ty == r.ty {
                if !n_r.visited {
                    score += visit_region((n_p, n_r), m);
                }
            } else {
                score.borders += 1;
            }
        } else {
            score.borders += 1;
        }
    }
    score.nodes += 1;
    score
}

fn visit_region_ex((p, r): (Point, Region), map: &mut HashMap<Point, Region>) -> Score {
    map.get_mut(&p).unwrap().visited = true;
    let mut score = Score::default();
    for (side, n_p) in p.neighbors() {
        let has_border = if let Some(&n_r) = map.get(&n_p) {
            if n_r.ty == r.ty {
                if !n_r.visited {
                    score += visit_region_ex((n_p, n_r), map);
                }
                false
            } else {
                true
            }
        } else {
            true
        };
        if has_border {
            map.get_mut(&p).unwrap().set_border(side);
            check_border(side, &mut score, map, p);
        }
    }
    score.nodes += 1;
    score
}

fn check_border(check: Side, score: &mut Score, map: &mut HashMap<Point, Region>, p: Point) {
    let (side1, side2) = match check {
        Side::Top => (Side::Left, Side::Right),
        Side::Bottom => (Side::Left, Side::Right),
        Side::Right => (Side::Top, Side::Bottom),
        Side::Left => (Side::Top, Side::Bottom),
    };
    if ![map.get(&p.neighbor(&side1)), map.get(&p.neighbor(&side2))]
        .into_iter()
        .filter_map(identity)
        .any(|i| i.has_border(check) && i.visited)
    {
        score.borders += 1;
    }
}

fn part_1(input: &str) -> impl Debug {
    let mut map = parse_map(input);
    let mut out = 0;
    while let Some(next_node) = find_next(&map) {
        let score = visit_region(next_node, &mut map);
        out += score.borders * score.nodes;
    }
    out
}

fn part_2(input: &str) -> impl Debug {
    let mut map = parse_map(input);
    let mut out = 0;
    while let Some(next_node) = find_next(&map) {
        let score = visit_region_ex(next_node, &mut map);
        println!("{next_node:?} -- {score:?}");
        out += score.borders * score.nodes;
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const TEST_INPUT2: &'static str = "AAAA
BBCD
BBCC
EEEC";

    #[test]
    fn test_1() {
        let actual = format!("{:?}", part_1(TEST_INPUT));
        assert_eq!(actual, "1930");
    }
    #[test]
    fn test_2() {
        let actual = format!("{:?}", part_2(TEST_INPUT2));
        assert_eq!(actual, "80");
    }
}
