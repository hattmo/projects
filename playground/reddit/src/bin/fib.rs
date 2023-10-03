use std::iter;

fn main() {
    for i in fib().take(6){
        println!("{}", i);
    }
}

fn fib() -> impl Iterator<Item = u128> {
    iter::successors(Some((0, 1)), |(prev, next)| Some((*next, prev + next)))
        .map(|(i, _)| i)
}

struct Foo {}
