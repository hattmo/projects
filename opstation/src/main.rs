// const STACK_SIZE: usize = 10 * 1024 * 1024;

use clap::Parser;
use opstation::create_container;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    manager: Option<String>,
}

fn main() {
    let arg = Args::parse();
    match create_container("/bin/bash", &[], "") {
        Ok(_) => {
            print!("exit successful")
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    };
}
