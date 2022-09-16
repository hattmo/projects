use treers::Tree;

fn main() {
    let mut tr = Tree::new(20);
    let node = tr.get_root_mut();
    let mut foo = node.parent().unwrap();
    foo.put_child(435);
}
