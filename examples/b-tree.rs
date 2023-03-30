use std::rc::Rc;

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>,
}
#[derive(Debug, Clone)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }
    fn append(&self, value: i32) -> Self {
        LinkedList {
            head: Some(Box::new(Node {
                value: value,
                next: self.head.clone(), // clone not borrow
            })),
        }
    }
}

fn main() {
    // let x = Node::new(1);
    let x = LinkedList::new().append(1).append(2).append(3);
    print!("{:?}", x);
    // print!("{}",x.get_value());
}
