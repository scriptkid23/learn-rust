use std::rc::Rc;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<Node<T>>>,
}
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }
    fn append(&self, data: T) -> Self {
        LinkedList {
            head: Some(Rc::new(Node {
                data: data,
                next: self.head.clone(),
            })),
        }
    }
}
fn main() {
    let list = LinkedList::new().append(3).append(4).append(10);
    println!("{:?}",list.head.unwrap().next.as_ref().unwrap().data)
}
