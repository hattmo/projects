use std::{io::Result as IoResult, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
struct TaskItem {
    name: String,
    commands: Vec<String>,
}

fn main() {
    let item = TaskItem::default();
    let out = serde_yaml::to_string(&item).unwrap();
    println!("{out}");
}

struct TaskFile {}
impl TaskFile {
    pub fn parse(path: &Path) -> IoResult<TaskFile> {
        Ok(Self {})
    }
}
