use std::fmt::Debug;

fn main() {
    helper::run("2024", "11", part_1, part_2);
}

fn part_1(_input: &str) -> impl Debug {
    "todo"
}
fn part_2(_input: &str) -> impl Debug {
    "todo"
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "TEST_INPUT";

    #[test]
    fn test_1() {
        let actual = format!("{:?}", part_1(TEST_INPUT));
        assert_eq!(actual, "\"todo\"");
    }
    #[test]
    fn test_2() {
        let actual = format!("{:?}", part_2(TEST_INPUT));
        assert_eq!(actual, "\"todo\"");
    }
}
