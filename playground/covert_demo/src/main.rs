#![feature(array_chunks)]

fn main() {
    let mut foo: u8 = 0xff;
    let bar: u16 = foo.into();
    let asdf = "hello".to_string();
    let hi = b"hello".to_vec();
    foo = bar.try_into().unwrap();
    let blah = foo_fn::<String>;
}
fn foo_fn<T: AsRef<str>>(a: T) {
    let a = a.as_ref();
    println!("{}", a);
}
