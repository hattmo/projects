#![feature(duration_millis_float)]
use anyhow::{anyhow, Result};
use clap::Parser;
use directories::UserDirs;
use reqwest::blocking;
use std::{fmt::Debug, fs, io::stdin, time::Instant};

#[derive(clap::Parser)]
struct Args {
    #[arg(long)]
    part1: bool,
    #[arg(long)]
    part2: bool,
}

pub fn get_input(year: &str, day: &str) -> Result<String> {
    let key = std::env::var("AOC_KEY").or(Err(anyhow!("Environment variable not set")))?;
    let cache = UserDirs::new()
        .ok_or(anyhow!("Could not get user dirs"))?
        .home_dir()
        .join(".aoc_cache")
        .join(year);
    if !cache.exists() {
        fs::create_dir_all(&cache).or(Err(anyhow!("Could not create dir")))?;
    };
    let input_file = cache.join(day.to_string());
    if input_file.exists() {
        return fs::read_to_string(input_file).or(Err(anyhow!("Could not read file")));
    } else {
        let out = blocking::Client::new()
            .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
            .header("cookie", format!("session={key}"))
            .send()
            .or(Err(anyhow!("Failed to send request")))?
            .error_for_status()
            .or_else(|e| Err(anyhow!("Got error code: {e}")))?
            .text()
            .or(Err(anyhow!("Could not parse body of request")))?;
        fs::write(input_file, &out)?;
        return Ok(out);
    };
}

pub fn run<T, U, F, V>(year: &str, day: &str, part_1: F, part_2: V)
where
    T: Debug,
    U: Debug,
    F: FnOnce(&str) -> T,
    V: FnOnce(&str) -> U,
{
    let mut args = Args::parse();
    if !args.part1 && !args.part2 {
        args.part1 = true;
        args.part2 = true;
    }
    match get_input(year, day) {
        Ok(input) => {
            if args.part1 {
                let now = Instant::now();
                let res = part_1(&input);
                let time = now.elapsed().as_millis_f64() / 1000.0;
                println!("part 1: {res:?} ({time:.4} secs)");
            }
            if args.part2 {
                let now = Instant::now();
                let res = part_2(&input);
                let time = now.elapsed().as_millis_f64() / 1000.0;
                println!("part 2: {res:?} ({time:.4} secs)");
            }
        }
        Err(err) => println!("Error getting input: {err}"),
    };
}

pub fn pause() {
    let _ = stdin().read_line(&mut String::new());
}
#[cfg(test)]
mod test {
    use crate::get_input;

    #[test]
    fn test() {
        match get_input("2021", "1") {
            Ok(val) => println!("{val}"),
            Err(err) => println!("{err}"),
        }
    }
}
