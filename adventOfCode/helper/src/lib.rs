use anyhow::{anyhow, Result};
use directories::UserDirs;
use reqwest::blocking;
use std::{
    fs::{self, File},
    io::{Read, Write},
};
pub fn get_input(year: &str, day: &str, use_cache: bool) -> Result<String> {
    let key = std::env::var("AOC_KEY").or(Err(anyhow!("Environment variable not set")))?;
    let cache = UserDirs::new()
        .ok_or(anyhow!("Could not get user dirs"))?
        .home_dir()
        .join(".aoc_cache")
        .join(year);
    if !cache.exists() {
        fs::create_dir_all(cache.clone()).or(Err(anyhow!("Could not create dir")))?;
    };
    let input_file = cache.join(day.to_string());
    if input_file.exists() && use_cache {
        let mut file = File::open(input_file).or(Err(anyhow!("Could not open cached file")))?;
        let mut out = String::new();
        file.read_to_string(&mut out)
            .or(Err(anyhow!("Could not read file to string")))?;
        return Ok(out);
    } else {
        let response = blocking::Client::new()
            .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
            .header("cookie", format!("session={key}"))
            .send()
            .or(Err(anyhow!("Failed to send request")))?
            .error_for_status()
            .or_else(|e| Err(anyhow!("Got error code: {e}")))?;
        let out = response
            .text()
            .or(Err(anyhow!("Could not parse body of request")))?;
        let mut file = File::create(input_file).or(Err(anyhow!("Could not create file")))?;
        file.write_all(out.as_bytes())
            .or(Err(anyhow!("Could not write to cache")))?;
        return Ok(out);
    };
}

#[cfg(test)]
mod test {
    use crate::get_input;

    #[test]
    fn test() {
        match get_input("2021", "1", true) {
            Ok(val) => println!("{val}"),
            Err(err) => println!("{err}"),
        }
    }
}
