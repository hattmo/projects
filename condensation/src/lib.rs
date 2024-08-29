use std::collections::HashMap;

mod vsphere {}

struct Resource {
    fields: HashMap<String, Box<dyn Res>
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
    pub name: Box<dyn Resolve<str>>,
}

impl VSphere {
    pub fn new(username: &str, password: &str) -> Self {
        VSphere {
            resources: Vec::new(),
        }
    }

    pub fn create_vm(&mut self, config: VMConfig) -> VM {
        VM {}
    }

    pub fn build(self) {
        let mut done = false;
        while !done {
            for item in self.resources {
                let _ = item.build();
            }
        }
    }
}

pub trait Resolve<T: ?Sized> {
    fn resolve(&self) -> Option<&T>;
}

impl Resolve<str> for &'static str {
    fn resolve(&self) -> Option<&str> {
        Some(*self)
    }
}

impl Resolve<str> for String {
    fn resolve(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

#[cfg(test)]
mod test {
    use crate::VMConfig;

    #[test]
    fn test() {
        VMConfig {
            name: Box::new("hello"),
        };
    }
}
