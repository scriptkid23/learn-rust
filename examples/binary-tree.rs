#[derive(PartialEq, Debug, Clone)]
struct BTree {
    root: Option<Box<Node>>,
}

#[derive(PartialEq, Debug, Clone)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value: value,
            left: None,
            right: None,
        }
    }
}
impl BTree {
    fn new() -> Self {
        BTree { root: None }
    }
    fn insertion(&mut self, value: i32) -> Self {
        println!("{:?}", self);
        if self.root == None {
            return BTree {
                root: Some(Box::new(Node::new(value))),
            };
        } else {
            let _b_tree_clone = self.root.clone();
            _b_tree_clone.unwrap().left = Some(Box::new(Node::new(3)));
            return BTree {
                root: _b_tree_clone,
            };
        }
    }
}
// 0 3
fn main() {
    let x = BTree::new().insertion(1).insertion(2).insertion(3);
}
