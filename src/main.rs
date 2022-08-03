#[derive(Debug)]


struct Node {
    value: i32,
    next: Option<Box<Node>>,
}
fn main() {
    // TODO: create me!
    let mut a = 10;
    let b = &mut a;

    *b = 100;
    
    let data = Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 2,
            next: None,
        })),
    };
    print!("{}",b);
    print!("{:?}", data.next.unwrap().value);
}
