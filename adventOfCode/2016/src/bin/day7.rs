#![allow(dead_code)]
#![allow(unused_variables)]
use helper::get_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2016", "7", true)?;
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
    Ok(())
}
fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(str::trim)
        .filter(|line| {
            let parts = line.split(&['[', ']'][..]);

            if parts
                .clone()
                .skip(1)
                .step_by(2)
                .find(|inside| {
                    inside
                        .chars()
                        .collect::<Vec<_>>()
                        .windows(4)
                        .find(|window| {
                            // println!("{:?}", window);
                            window[0] == window[3]
                                && window[1] == window[2]
                                && window[0] != window[1]
                        })
                        .is_some()
                })
                .is_some()
            {
                return false;
            }
            parts
                .step_by(2)
                .find(|outside| {
                    outside
                        .chars()
                        .collect::<Vec<_>>()
                        .windows(4)
                        .find(|window| {
                            // println!("{:?}", window);
                            window[0] == window[3]
                                && window[1] == window[2]
                                && window[0] != window[1]
                        })
                        .is_some()
                })
                .is_some()
        })
        .count()
}
fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(str::trim)
        .filter(|line| {
            let parts = line.split(&['[', ']'][..]);
            let outside = parts.clone().into_iter().step_by(2);
            let mut inside = parts.into_iter().skip(1).step_by(2);
            let potentials = outside.fold(Vec::<String>::new(), |mut acc, part| {
                acc.extend(
                    part.chars()
                        .collect::<Vec<_>>()
                        .windows(3)
                        .filter_map(|win| {
                            if win[0] == win[2] && win[1] != win[0] {
                                Some([win[1], win[0], win[1]].iter().collect())
                            } else {
                                None
                            }
                        }),
                );
                acc
            });
            inside
                .find(|part| {
                    potentials
                        .iter()
                        .find(|potential| part.contains(*potential))
                        .is_some()
                })
                .is_some()
        })
        .count()
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    #[test]
    fn test1() {
        assert_eq!(part_1("abba[mnop]qrst"), 1)
    }
    #[test]
    fn test2() {
        assert_eq!(part_1("abcd[bddb]xyyx"), 0)
    }
    #[test]
    fn test3() {
        assert_eq!(part_1("aaaa[qwer]tyui"), 0)
    }
    #[test]
    fn test4() {
        assert_eq!(part_1("ioxxoj[asdfgh]zxcvbn"), 1)
    }

    #[test]
    fn test5() {
        assert_eq!(part_2("aba[bab]xyz"), 1)
    }

    #[test]
    fn test6() {
        assert_eq!(part_2("xyx[xyx]xyx"), 0)
    }
}
