#![feature(iter_array_chunks)]
use helper::get_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input("2016", "3", true)?;
    part1(&input);
    part2(&input);
    Ok(())
}

fn valid_triangle(sides: &[i32; 3]) -> bool {
    let [first, second, third] = sides;
    return first + second > *third && second + third > *first && third + first > *second;
}

fn part1(input: &str) {
    let result = input
        .lines()
        .map(|item| {
            item.split(" ")
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(|i| i.parse::<i32>().expect("error parsing input"))
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap()
        })
        .filter(valid_triangle)
        .count();

    println!("part1: {result}");
}

fn part2(input: &str) {
    let result = input
        .lines()
        .map(|i| {
            i.split(" ")
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(|item| item.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap()
        })
        .array_chunks::<3>()
        .map(|chunk: [[i32; 3]; 3]| {
            let mut out: [[i32; 3]; 3] = [[0; 3]; 3];
            for i in 0..3 {
                for j in 0..3 {
                    out[i][j] = chunk[j][i];
                }
            }
            return out;
        })
        .flatten()
        .filter(valid_triangle)
        .count();
    println!("part2 {result}");

    // .array_chunks::<3>().map(|chunk| {
    //     chunk.into_iter();
    //     })
    // });
}
