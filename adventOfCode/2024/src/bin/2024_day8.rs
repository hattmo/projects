#![feature(cmp_minmax)]
use std::{
    cmp::{max, minmax},
    collections::{HashMap, HashSet},
    fmt::Debug,
};

fn main() {
    helper::run("2024", "8", part_1, part_2);
}

fn parse_map(input: &str) -> ((isize, isize), HashMap<char, Vec<(isize, isize)>>) {
    let mut out = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (row, line) in input.lines().enumerate() {
        height = max(row, height);
        for (col, item) in line.chars().enumerate() {
            if item != '.' {
                width = max(col, width);
                let entry = out.entry(item).or_insert(Vec::new());
                entry.push((col as isize, row as isize));
            }
        }
    }
    ((width.try_into().unwrap(), height.try_into().unwrap()), out)
}

fn antinode(left: &(isize, isize), right: &(isize, isize)) -> ((isize, isize), (isize, isize)) {
    let [&(lx, ly), &(rx, ry)] = minmax(left, right);
    let x_dist: isize = lx.abs_diff(rx).try_into().unwrap();
    let y_dist: isize = ly.abs_diff(ry).try_into().unwrap();
    let ol_x = lx - x_dist;
    let or_x = rx + x_dist;
    let ol_y;
    let or_y;
    if ly > ry {
        ol_y = ly + y_dist;
        or_y = ry - y_dist;
    } else {
        ol_y = ly - y_dist;
        or_y = ry + y_dist;
    }
    ((ol_x, ol_y), (or_x, or_y))
}

fn antinode_ex(
    &(mut lx, mut ly): &(isize, isize),
    &(mut rx, mut ry): &(isize, isize),
    width: isize,
    height: isize,
) -> Vec<(isize, isize)> {
    let x_bound = 0..=width;
    let y_bound = 0..=height;
    let mut out = Vec::new();
    let xdif = lx - rx;
    let ydif = ly - ry;
    while x_bound.contains(&lx) && y_bound.contains(&ly) {
        out.push((lx, ly));
        lx -= xdif;
        ly -= ydif;
    }
    while x_bound.contains(&rx) && y_bound.contains(&ry) {
        out.push((rx, ry));
        rx += xdif;
        ry += ydif;
    }
    out
}

fn part_1(input: &str) -> impl Debug {
    let ((width, height), antene) = parse_map(input);
    let mut nodes = HashSet::new();
    for (_, coords) in antene {
        for (l, r) in coords.combo() {
            let (al, ar) = antinode(l, r);
            nodes.insert(al);
            nodes.insert(ar);
        }
    }
    nodes
        .into_iter()
        .filter(move |&(nx, ny)| nx >= 0 && nx <= width && ny >= 0 && ny <= height)
        .count()
}
fn part_2(input: &str) -> impl Debug {
    let ((width, height), antene) = parse_map(input);
    let mut nodes = HashSet::new();
    for (_, coords) in antene {
        for (l, r) in coords.combo() {
            let new_nodes = antinode_ex(l, r, width, height);
            nodes.extend(new_nodes);
        }
    }
    nodes.len()
}

struct ComboIter<'a, T> {
    left: usize,
    right: usize,
    inner: &'a [T],
}
trait Combination<'a, T> {
    fn combo(&'a self) -> ComboIter<'a, T>;
}

impl<'a, T: 'a, U> Combination<'a, T> for U
where
    U: AsRef<[T]>,
{
    fn combo(&'a self) -> ComboIter<'a, T> {
        ComboIter {
            left: 0,
            right: 0,
            inner: self.as_ref(),
        }
    }
}

impl<'a, T> Iterator for ComboIter<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let Self { left, right, inner } = self;
        *right += 1;
        if *right >= inner.len() {
            *left += 1;
            *right = *left + 1;
        }
        if *right >= inner.len() {
            return None;
        }
        Some((&self.inner[*left], &self.inner[*right]))
    }
}
