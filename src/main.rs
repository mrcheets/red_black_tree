use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    value: u32,
    parent: Link,
    left: Link,
    right: Link,
}

impl Node {
    fn new(value: u32) -> Self {
        Self {
            value: value,
            parent: None,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, parent: &Rc<RefCell<Node>>, mut node: Node) {
        if self.value <= node.value {
            match self.right {
                Some(ref parent_node) => {
                    parent_node.borrow_mut().insert(parent_node, node);
                }
                None => {
                    node.parent = Some(parent.clone());
                    self.right = Some(Rc::new(RefCell::new(node)));
                }
            }
        } else {
            match self.left {
                Some(ref parent_node) => {
                    parent_node.borrow_mut().insert(parent_node, node);
                }
                None => {
                    node.parent = Some(parent.clone());
                    self.left = Some(Rc::new(RefCell::new(node)));
                }
            }
        }
    }

}

#[derive(Debug)]
struct RedBlackTree {
    root: Link,
}

impl RedBlackTree {
    fn new() -> Self {
        Self {
            root: None,
        }
    }

    fn insert_node(&mut self, node: Node) {
        match self.root {
            Some(ref root_node) => {
                root_node.borrow_mut().insert(root_node, node);
            }
            None => {
                self.root = Some(Rc::new(RefCell::new(node)));
            }
        }
    }
}

fn main() {
    let mut rb_tree = RedBlackTree::new();
    let node1 = Node::new(3);
    let node2 = Node::new(5);
    rb_tree.insert_node(node1);
    rb_tree.insert_node(node2);
    println!("{:?}", rb_tree);

}
