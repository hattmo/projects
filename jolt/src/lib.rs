use std::{any::Any, collections::HashMap};

type RenderFn = fn(props: HashMap<&'static str, Box<dyn Any>>, ctx: &mut Context) -> Box<dyn Node>;

pub struct Context;

impl Context {
    pub fn use_state<T>(&mut self, val: T) -> (T) {
        val
    }
}

pub trait Node {}

pub struct Component {
    render_fn: RenderFn,
    ctx: Context,
}

impl Component {
    pub fn new(render_fn: RenderFn) -> Self {
        Self {
            render_fn,
            ctx: Context,
        }
    }

    pub fn render(&mut self) -> Box<dyn Node> {
        (self.render_fn)(HashMap::new(), &mut self.ctx)
    }
}
