use std::ptr::{null_mut};

#[derive(PartialEq, Debug)]
enum Color {
    RED,
    BLACK,
}

#[derive(Debug)]
struct Node {
    data: u32,
    color: Color,
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
            color: Color::RED,
            parent: null_mut(),
            left: null_mut(),
            right: null_mut(),
        }
    }

    fn insert(&mut self, node: &mut Node) {
        if self.data <= node.data {
            if self.right.is_null() {
                    node.color = Color::RED;
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
                    node.color = Color::RED;
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
            node.color = Color::BLACK;
            self.root = node;
        } else {
            unsafe {
                let root_node = &mut (*self.root);
                root_node.insert(node);

            }
        }
    }

    fn balance_tree(&mut self, start_node: &mut Node) {
        if start_node.parent.is_null(){
            return;
        } else {
            unsafe {
                let parent = &(*start_node.parent);
                if parent.color == Color::BLACK {
                    return;
                } else {
                    let uncle = self.get_uncle(start_node);
                    println!("Parent color is red");
                    println!("Uncle: {:?}", uncle);

                }
            }
        }
    }

    fn get_uncle(&mut self, node: &mut Node) -> *mut Node {
        unsafe {
            let parent = &(*node.parent);
            let grand_parent = parent.parent;
            if grand_parent.is_null() {
                null_mut()
            } else {
                if parent.data >= (*grand_parent).data {
                    (*grand_parent).left
                } else {
                    (*grand_parent).right
                }
            }
        }
    }
}


fn main() {
    let mut rb_tree = RedBlackTree::new();
    let mut node1 = Node::new(3);
    let mut node2 = Node::new(5);
    let mut node3 = Node::new(2);
    let mut node4 = Node::new(4);
    rb_tree.insert_node(&mut node1);
    rb_tree.balance_tree(&mut node1);
    rb_tree.insert_node(&mut node2);
    rb_tree.balance_tree(&mut node2);
    rb_tree.insert_node(&mut node3);
    rb_tree.balance_tree(&mut node3);
    rb_tree.insert_node(&mut node4);
    rb_tree.balance_tree(&mut node4);
    
    /*println!("{:?}", rb_tree);
    println!("{:?}", unsafe{ &(*rb_tree.root) });
    println!("{:?}", unsafe{ &(*(*rb_tree.root).right) });
    println!("{:?}", unsafe{ &(*(*rb_tree.root).left) });*/
}
