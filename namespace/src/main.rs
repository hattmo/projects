// const STACK_SIZE: usize = 10 * 1024 * 1024;

use docker_playground::create_container;

fn main() {
    match create_container("/bin/bash", &[], "") {
        Ok(_) => {
            print!("exit successful")
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    };
}