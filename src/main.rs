use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    data: u32,
    parent: Link,
    left: Link,
    right: Link,
}

impl Node {
    fn new(data: u32) -> Self {
        Self {
            data: data,
            parent: None,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, parent: Link, node: Rc<RefCell<Node>>) {
        if self.data <= node.borrow().data {
            match &self.right {
                Some(parent_node) => {
                    parent_node.borrow_mut().insert(Some(Rc::clone(parent_node)), node);
                }
                None => {
                    node.borrow_mut().parent = parent;
                    self.right = Some(node);
                }
            }
        } else {
            match &self.left {
                Some(parent_node) => {
                    parent_node.borrow_mut().insert(Some(Rc::clone(parent_node)), node);
                }
                None => {
                    node.borrow_mut().parent = parent;
                    self.left = Some(node);
                }
            }
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let parent = match &self.parent {
            Some(p) => format!("Node with value: {}", p.borrow().data),
            None => format!("None"),
        };
        f.debug_struct("Node")
            .field("data", &self.data)
            .field("parent", &parent)
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
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

    fn insert_node(&mut self, node: Rc<RefCell<Node>>) {
        match self.root {
            Some(ref root_node) => {
                root_node.borrow_mut().insert(Some(Rc::clone(root_node)), node);
            }
            None => {
                self.root = Some(node);
            }
        }
    }
}

fn main() {
    let mut rb_tree = RedBlackTree::new();
    let node1 = Rc::new(RefCell::new(Node::new(3)));
    let node2 = Rc::new(RefCell::new(Node::new(5)));
    let node3 = Rc::new(RefCell::new(Node::new(4)));
    rb_tree.insert_node(node1.clone());
    rb_tree.insert_node(node2.clone());
    rb_tree.insert_node(node3.clone());
    println!("{:?}", rb_tree);
    println!("{:?}", node3);

}
