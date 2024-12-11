use std::fmt::Debug;

fn main() {
    helper::run("2024", "4", part_1, part_2);
}

fn part_1(input: &str) -> impl Debug {
    let b: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let w: isize = b.len().try_into().unwrap();
    let h: isize = b[0].len().try_into().unwrap();
    let mut count = 0;
    for x in 0..w {
        for y in 0..h {
            for (xv, yv) in [(0, 1), (1, 0), (1, 1), (1, -1)] {
                let word = (
                    b.xy(x, y),
                    b.xy(x + (1 * xv), y + (1 * yv)),
                    b.xy(x + (2 * xv), y + (2 * yv)),
                    b.xy(x + (3 * xv), y + (3 * yv)),
                );
                match word {
                    (Some(&'X'), Some(&'M'), Some(&'A'), Some(&'S')) => count += 1,
                    (Some(&'S'), Some(&'A'), Some(&'M'), Some(&'X')) => count += 1,
                    _ => {}
                }
            }
        }
    }
    count
}

fn part_2(input: &str) -> impl Debug {
    let b: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let h: isize = b.len().try_into().unwrap();
    let w: isize = b[0].len().try_into().unwrap();
    let mut count = 0;
    for y in 0..h {
        for x in 0..w {
            let word = [
                b.xy(x, y),
                b.xy(x - 1, y - 1),
                b.xy(x - 1, y + 1),
                b.xy(x + 1, y + 1),
                b.xy(x + 1, y - 1),
            ];
            let add = match word {
                [Some(&'A'), rest @ ..] => match rest {
                    [Some('M'), Some('M'), Some('S'), Some('S')] => 1,
                    [Some('S'), Some('S'), Some('M'), Some('M')] => 1,
                    [Some('M'), Some('S'), Some('S'), Some('M')] => 1,
                    [Some('S'), Some('M'), Some('M'), Some('S')] => 1,
                    _ => 0,
                },
                _ => 0,
            };
            count += add;
        }
    }
    count
}

trait TwoDimentional<T> {
    fn xy(&self, x: isize, y: isize) -> Option<&T>;
}

impl<T> TwoDimentional<T> for Vec<Vec<T>> {
    fn xy(&self, x: isize, y: isize) -> Option<&T> {
        if x < 0 || y < 0 {
            return None;
        }
        self.get(y as usize).and_then(|i| i.get(x as usize))
    }
}

#[cfg(test)]
mod test {
    use crate::part_2;

    #[test]
    fn test_part2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let actual = format!("{:?}", part_2(input));
        let expected = "9";
        assert_eq!(actual, expected);
    }
}
