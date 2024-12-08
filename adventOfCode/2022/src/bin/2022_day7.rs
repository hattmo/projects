#![allow(dead_code)]
#![allow(unused_variables)]
use helper::get_input;
use itertools::Itertools;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2022", "7", true)?;
    println!("part1 {:?}", part_1(&input));
    println!("part2 {:?}", part_2(&input));
    Ok(())
}

#[derive(Debug)]
struct Directory {
    parent: usize,
    children: Vec<(String, usize)>,
    size: usize,
}

impl Directory {
    fn dir_size(&self, dirs: &Vec<Directory>) -> usize {
        let mut out = self.size;
        for index in self.children.iter().map(|(_, index)| index) {
            out += dirs.get(*index).unwrap().dir_size(dirs);
        }
        out
    }
}
fn parse_dirs(input: &str) -> Vec<Directory> {
    let mut dirs = Vec::<Directory>::new();
    dirs.push(Directory {
        children: Vec::new(),
        parent: 0,
        size: 0,
    });
    let mut on_index = 0;
    for line in input.lines().map(str::trim) {
        match line.split(' ').collect_vec()[..] {
            ["$", "cd", "/"] => {
                on_index = 0;
            }
            ["$", "cd", ".."] => {
                on_index = dirs.get(on_index).unwrap().parent;
            }
            ["$", "cd", dir] => {
                on_index = *dirs
                    .get(on_index)
                    .unwrap()
                    .children
                    .iter()
                    .find_map(|(name, index)| if *name == dir { Some(index) } else { None })
                    .unwrap();
            }
            ["$", "ls"] => {}
            ["dir", new_dir] => {
                let new_index = dirs.len();
                dirs.push(Directory {
                    children: Vec::new(),
                    parent: on_index,
                    size: 0,
                });
                dirs.get_mut(on_index)
                    .unwrap()
                    .children
                    .push((new_dir.to_owned(), new_index));
            }
            [file_size, _] => {
                dirs.get_mut(on_index).unwrap().size += file_size.parse::<usize>().unwrap();
            }
            _ => panic!(),
        }
    }
    dirs
}
fn part_1(input: &str) -> usize {
    // println!("{:?}", dirs);
    let dirs = parse_dirs(input);
    dirs.iter()
        .filter_map(|item| {
            let size = item.dir_size(&dirs);
            if size <= 100000 {
                Some(size)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let dirs = parse_dirs(input);
    const TOTAL: usize = 70000000;
    let need = 30000000 - (TOTAL - dirs[0].dir_size(&dirs));
    dirs.iter()
        .filter_map(|item| {
            let size = item.dir_size(&dirs);
            if size >= need {
                Some(size)
            } else {
                None
            }
        })
        .sorted()
        .next()
        .unwrap()
}

#[cfg(test)]
mod test {

    use crate::{part_1, part_2};

    const TEST_DATA: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
    #[test]
    fn test1() {
        let expected = 95437;
        let actual = part_1(TEST_DATA);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let expected = 24933642;
        let actual = part_2(TEST_DATA);
        assert_eq!(expected, actual);
    }

    #[test]
    fn play() {}
}

//48008081
