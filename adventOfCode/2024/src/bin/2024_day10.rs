use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    usize,
};

fn main() {
    helper::run("2024", "10", part_1, part_2);
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn parse_map(input: &str) -> HashMap<Point, usize> {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let h = c.to_digit(10).unwrap() as usize;
                (Point { x: x + 1, y: y + 1 }, h)
            })
        })
        .flatten()
        .collect()
}

fn find_trail_heads(map: &HashMap<Point, usize>) -> Vec<(Point, usize)> {
    map.iter()
        .filter_map(|(&k, &v)| (v == 0).then_some((k, v)))
        .collect()
}

fn find_peaks(point: (Point, usize), map: &HashMap<Point, usize>) -> HashSet<Point> {
    get_neighbors(point, map)
        .into_iter()
        .map(|p| {
            if p.1 == 9 {
                HashSet::from([p.0])
            } else {
                find_peaks(p, map)
            }
        })
        .flatten()
        .collect()
}
fn find_paths(point: (Point, usize), map: &HashMap<Point, usize>) -> usize {
    get_neighbors(point, map)
        .into_iter()
        .map(|p| if p.1 == 9 { 1 } else { find_paths(p, map) })
        .sum()
}

fn get_neighbors(
    (Point { x, y }, val): (Point, usize),
    map: &HashMap<Point, usize>,
) -> Vec<(Point, usize)> {
    [
        Point { x: x - 1, y },
        Point { x: x + 1, y },
        Point { x, y: y - 1 },
        Point { x, y: y + 1 },
    ]
    .into_iter()
    .filter_map(|p| map.get(&p).and_then(|&h| (h == val + 1).then_some((p, h))))
    .collect()
}

fn part_1(input: &str) -> impl Debug {
    let map = parse_map(input);
    let trail_heads = find_trail_heads(&map);
    trail_heads
        .into_iter()
        .map(move |trail_head| find_peaks(trail_head, &map).len())
        .sum::<usize>()
}
fn part_2(input: &str) -> impl Debug {
    let map = parse_map(input);
    let trail_heads = find_trail_heads(&map);
    trail_heads
        .into_iter()
        .map(move |trail_head| find_paths(trail_head, &map))
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const TEST_INPUT: &'static str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_1() {
        let actual = format!("{:?}", part_1(TEST_INPUT));
        assert_eq!(actual, "36");
    }
    #[test]
    fn test_2() {
        let actual = format!("{:?}", part_2(TEST_INPUT));
        assert_eq!(actual, "81");
    }
}
