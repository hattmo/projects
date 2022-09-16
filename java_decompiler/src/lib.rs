#![feature(iter_intersperse)]
// pub mod class;
pub mod class;
pub mod constant;
pub mod parsing;
#[cfg(test)]
mod util;

#[cfg(test)]
mod test {
    use crate::class::Class;
    use crate::util::Pretty;
    use std::io::{Read, Result};

    #[test]
    fn print_test() -> Result<()> {
        let mut file = std::fs::File::open("test/Test.class").unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        if let Ok((_, class)) = Class::from_slice(&buf).map_err(|e| {
            e.map(|i| {
                println!("{}", i.input.pretty());
                i
            })
        }) {
            print!("{class}",);
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "parse error",
            ))
        }
    }
}
