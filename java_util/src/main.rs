use std::error::Error;

use java_util::Class;
pub fn main() -> Result<(), Box<dyn Error>> {
    let foo = [1u8, 2, 3, 4, 5];
    let class = Class::from_reader(foo.as_slice())?;
    println!("hello");
    Ok(())
}
