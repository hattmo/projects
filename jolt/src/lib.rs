pub struct Context;

pub struct Node;

trait RootComponent {
    fn render(&mut self, ctx: &mut Context) -> Node;
}

impl<T> RootComponent for T
where
    T: Component<()>,
{
    fn render(&mut self, ctx: &mut Context) -> Node {
        self.render(ctx, ())
    }
}

trait Component<T> {
    fn render(&mut self, ctx: &mut Context, props: T) -> Node;
}
