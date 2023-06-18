use std::ptr::{null_mut};

#[derive(PartialEq)]
enum Color {
    RED,
    BLACK,
}

#[derive(Debug)]
struct Node {
    data: u32,
    parent: *mut Node,
    left: *mut Node,
    right: *mut Node,
}

#[derive(Debug)]
struct RedBlackTree {
    root: *mut Node,
}

impl Node {
    fn new(data: u32) -> Self {
        Self {
            data: data,
            parent: null_mut(),
            left: null_mut(),
            right: null_mut(),
        }
    }

    fn insert(&mut self, node: &mut Node) {
        if self.data <= node.data {
            if self.right.is_null() {
                    node.parent = self;
                    self.right = node;
            } else {
                unsafe {
                    let parent = &mut (*self.right);
                    parent.insert(node);
                }
            }
        } else {
            if  self.left.is_null() {
                    node.parent = self;
                    self.left = node;
            } else {
                unsafe { 
                    let parent = &mut (*self.left);
                    parent.insert(node); 
                }
            }
        }
    }
}

impl RedBlackTree {
    fn new() -> Self {
        Self {
            root: null_mut()
        }
    }

    fn insert_node(&mut self, node: &mut Node) {
        if self.root.is_null() {
            self.root = node;
        } else {
            unsafe {
                let root_node = &mut (*self.root);
                root_node.insert(node);

            }
        }
    }

    fn balance_tree(&mut self, start_node: &mut Node) {

    }
}


fn main() {
    let mut rb_tree = RedBlackTree::new();
    let mut node1 = Node::new(3);
    let mut node2 = Node::new(5);
    let mut node3 = Node::new(2);
    rb_tree.insert_node(&mut node1);
    rb_tree.insert_node(&mut node2);
    rb_tree.insert_node(&mut node3);
    println!("{:?}", rb_tree);
    println!("{:?}", unsafe{ &(*rb_tree.root) });
    println!("{:?}", unsafe{ &(*(*rb_tree.root).right) });
    println!("{:?}", unsafe{ &(*(*rb_tree.root).left) });
}
