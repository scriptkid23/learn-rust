fn change(num: &mut i32) {
    *num = 3;
}

fn main() {
    let mut num = 10;
    change(&mut num);
    println!("{:?}", num);
}
