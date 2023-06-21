use std::env::args;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let mut args = args();
    let Some(arg) = args.nth(1) else {
        println!("Must give a file arg");
        return;
    };
    let file = BufReader::new(File::open(arg).unwrap());
    print!("printf \"%b\" ");
    file.bytes()
        .map(|b| b.unwrap())
        .for_each(|b| print!("\\\\0{:o}", b));
    println!(" > out.txt");
}
