use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::VecDeque,
    error::Error,
    fmt::Debug,
    u128,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = helper::get_input("2024", "9", true)?;
    let res = part1(&input);
    println!("part1: {res:?}");
    let res = part2(&input);
    println!("part2: {res:?}");
    Ok(())
}
#[derive(Debug)]
enum Block {
    Blank(u128),
    File { id: u128, size: u128 },
}

fn calc_sum(id: u128, size: u128, index: &mut u128, sum: &mut u128) {
    for i in *index..*index + size {
        *sum += i * id;
    }
    *index += size;
}

fn part1(input: &str) -> impl Debug + use<'_> {
    let (_, _, mut blocks) = input.chars().fold(
        (true, 0, VecDeque::new()),
        |(is_file, mut id, mut blocks), item| {
            if !item.is_digit(10) {
                return (is_file, id, blocks);
            }
            let val: u128 = item.to_digit(10).unwrap().into();
            if is_file {
                blocks.push_back(Block::File { id, size: val });
                id += 1;
            } else {
                blocks.push_back(Block::Blank(val));
            }
            (!is_file, id, blocks)
        },
    );
    let mut sum: u128 = 0;
    let mut index: u128 = 0;
    'outer: loop {
        match blocks.pop_front() {
            Some(Block::Blank(mut b_size)) => loop {
                match blocks.pop_back() {
                    Some(Block::Blank(_)) => continue,
                    Some(Block::File { id, size: f_size }) => match f_size.cmp(&b_size) {
                        Less => {
                            calc_sum(id, f_size, &mut index, &mut sum);
                            b_size -= f_size;
                        }
                        Equal => {
                            calc_sum(id, b_size, &mut index, &mut sum);
                            break;
                        }
                        Greater => {
                            calc_sum(id, b_size, &mut index, &mut sum);
                            blocks.push_back(Block::File {
                                id,
                                size: f_size - b_size,
                            });
                            break;
                        }
                    },
                    None => break 'outer,
                };
            },
            Some(Block::File { id, size }) => {
                calc_sum(id, size, &mut index, &mut sum);
            }
            None => break,
        }
    }
    sum
}

#[derive(Debug, Clone, Copy)]
struct File {
    moved: bool,
    id: u128,
    size: u128,
}
#[derive(Debug)]
struct Blank {
    space: u128,
    files: Vec<File>,
}

fn part2(input: &str) -> impl Debug + use<'_> {
    let (_, _, mut files, mut blanks) = input.chars().fold(
        (true, 0, Vec::new(), Vec::new()),
        |(is_file, mut id, mut files, mut blanks), item| {
            if !item.is_digit(10) {
                return (is_file, id, files, blanks);
            }
            let val: u128 = item.to_digit(10).unwrap().into();
            if is_file {
                files.push(File {
                    moved: false,
                    id,
                    size: val,
                });
                id += 1;
            } else {
                blanks.push(Blank {
                    space: val,
                    files: Vec::new(),
                });
            }
            (!is_file, id, files, blanks)
        },
    );
    for (i, file) in files.iter_mut().enumerate().rev() {
        for blank in blanks.iter_mut().take(i) {
            if blank.space >= file.size {
                blank.space -= file.size;
                blank.files.push(file.clone());
                file.moved = true;
                break;
            }
        }
    }
    let mut index = 0;
    let mut sum = 0;
    for (file, blank) in files.into_iter().zip(blanks) {
        if !file.moved {
            calc_sum(file.id, file.size, &mut index, &mut sum);
        } else {
            index += file.size;
        }

        for moved_file in blank.files {
            calc_sum(moved_file.id, moved_file.size, &mut index, &mut sum);
        }
        index += blank.space
    }
    sum
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn test() {
        let input = "2333133121414131402";
        let res = part1(input);
        println!("test1: {res:?}");
        let res = part2(input);
        println!("test2: {res:?}");
    }
}
