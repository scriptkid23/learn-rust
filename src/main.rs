fn change(num: &mut i32) {
    *num = 3;
}

struct Character {
    name: String,
}

impl Drop for Character {
    fn drop(&mut self) {
        print!("{} was dropped", self.name);
    }
}
fn main() {
    let steve = Character {
        name: String::from("123"),
    };


    
}
