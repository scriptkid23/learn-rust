#[derive(Debug)]
struct Node {
    data: u32,
    next: Option<Box<Node>>,
}
fn main() {
    let head = Node {
        data: 3,
        next: Option::from(Box::new(Node {
            data: 4,
            next: None,
        })),
    };

    println!("{:?}", head);
}
