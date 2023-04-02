
#[derive(Debug, Clone, PartialEq, Eq)]
struct BinarySearchTree {
    root: Option<Box<Node>>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        return Node {
            value: value,
            left: None,
            right: None,
        };
    }
}
impl BinarySearchTree {
    fn new() -> Self {
        return BinarySearchTree { root: None };
    }
    fn insertion(&mut self, value: i32) {
        if self.root == None {
            self.root = Some(Box::new(Node::new(value)));
        } else {
            Self::push_node(&mut self.root, value);
        }
    }
    fn push_node(root: &mut Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        if root.clone() == None {
            return Some(Box::new(Node::new(value)));
        }
        if root.clone().unwrap().value <= value {
             root.as_mut().unwrap().right = Self::push_node(&mut root.as_mut().unwrap().right, value);
        } else {
            root.as_mut().unwrap().left = Self::push_node(&mut root.as_mut().unwrap().left, value);
        }
        return root.clone();
    }
}
fn main() {
    let mut x = BinarySearchTree::new();
    x.insertion(1);
    x.insertion(0);
    x.insertion(2);
    x.insertion(3);
    x.insertion(1);
    println!("{:?}", x);
}
