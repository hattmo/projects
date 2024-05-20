use std::{
    cell::{Cell, RefCell},
    ops::Deref,
    rc::Rc,
};
mod vsphere {

}
pub struct VSphere {
    resources: Vec<Box<dyn Build>>,
}

enum BuildResult {
    Unresolved,
    Done,
}
enum BuildError {
    Error,
}
trait Build {
    fn build(&self) -> Result<BuildResult, BuildError>;
}

pub struct VM {}

impl Build for VM {
    fn build(&self) -> Result<BuildResult, BuildError> {
        todo!()
    }
}

pub struct VMConfig {
    pub name: BoundString,
}

impl VSphere {
    pub fn new() -> Self {
        VSphere {
            resources: Vec::new(),
        }
    }

    pub fn create_vm(&mut self, config: VMConfig) -> VM {
        VM {}
    }

    pub fn build(self) {
        for item in self.resources {
            let _ = item.build();
        }
    }
}

trait Resolve<T: ?Sized> {
    fn resolve(&self) -> Option<&T>;
}

pub struct BoundString {
    inner: Rc<RefCell<Option<String>>>,
}

impl<T> From<T> for BoundString
where
    T: ToString,
{
    fn from(value: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(Some(value.to_string()))),
        }
    }
}

impl Resolve<str> for BoundString {
    fn resolve(&self) -> Option<&str> {
        self.
    }
}

#[cfg(test)]
mod test {
    use crate::VMConfig;

    #[test]
    fn test() {
        VMConfig {
            name: "hello".into(),
        };
    }
}
