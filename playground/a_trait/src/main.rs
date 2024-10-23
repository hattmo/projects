use std::{future::Future, pin::Pin, task::Poll};

fn main() {
    let foo: Box<dyn TheTrait> = Box::new(TheTraitImpl);
    let blah = async {
        foo.foo_bar().await;
    };
}

struct TheTraitImpl;

impl TheTrait for TheTraitImpl {
    fn foo_bar(&self) -> FooFuture {
        todo!()
    }
}

trait TheTrait {
    fn foo_bar(&self) -> FooFuture;
}

struct FooFuture;

impl Future for FooFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
