use rand::random;
use std::collections::HashMap;
pub struct Tree<T> {
    data: HashMap<usize, TreeNode<T>>,
    root: usize,
}

struct TreeNode<T> {
    parent: Option<usize>,
    children: Vec<usize>,
    data: T,
}

impl<T> Tree<T> {
    pub fn new(initial: T) -> Tree<T> {
        let root = random();
        let mut data = HashMap::new();
        data.insert(
            root,
            TreeNode {
                parent: None,
                children: Vec::new(),
                data: initial,
            },
        );
        Tree { root, data }
    }

    pub fn get_root_mut(&mut self) -> TreeCursorMut<T> {
        let root = self.root.clone();
        TreeCursorMut {
            tree: self,
            index: root,
        }
    }
    pub fn get_root(&self) -> TreeCursor<T> {
        TreeCursor {
            index: self.root,
            tree: self,
        }
    }
}

pub struct TreeCursorMut<'a, T> {
    tree: &'a mut Tree<T>,
    index: usize,
}

pub struct TreeCursor<'a, T> {
    tree: &'a Tree<T>,
    index: usize,
}

impl<'a, T> TreeCursorMut<'a, T> {
    pub fn data(&mut self) -> &mut T {
        &mut self.tree.data.get_mut(&self.index).unwrap().data
    }
    pub fn parent(self) -> Option<TreeCursorMut<'a, T>> {
        let parent_index = self.tree.data.get(&self.index).unwrap().parent?;
        Some(TreeCursorMut {
            tree: self.tree,
            index: parent_index,
        })
    }
    pub fn first_child(self) -> Option<TreeCursorMut<'a, T>> {
        let child_index = self
            .tree
            .data
            .get(&self.index)
            .unwrap()
            .children
            .first()?
            .clone();
        Some(TreeCursorMut {
            tree: self.tree,
            index: child_index,
        })
    }

    pub fn put_child(&mut self, data: T) {
        let tree_data = &mut self.tree.data;
        let parent_index = self.index;
        let index = random();
        tree_data.insert(
            index,
            TreeNode {
                parent: Some(parent_index),
                children: Vec::new(),
                data: data,
            },
        );
        tree_data
            .get_mut(&parent_index)
            .unwrap()
            .children
            .push(index);
    }
    pub fn iter(&self) -> TreeIter<T> {
        TreeIter {
            cursor: self,
            index: 0,
        }
    }
}

pub struct TreeIter<'a, T> {
    cursor: &'a TreeCursorMut<'a, T>,
    index: usize,
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = TreeCursor<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.cursor.tree.data.get(&self.cursor.index).unwrap();
        let child_index = node.children.get(self.index)?;
        self.index += 1;
        Some(TreeCursor {
            tree: self.cursor.tree,
            index: *child_index,
        })
    }
}
