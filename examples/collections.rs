use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
struct Dummy;

#[derive(Clone, Debug)]
struct Fruit {
    name: String
}
/*
    nếu sử dụng 
    #[derive(Debug)]
    struct Fruit {
        name: String
    }
    không sử dụng được clone trong hàm main()
*/

fn main() {
    // TODO: array example
    let array: [u128; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", array);
    // TODO: tuple example
    let tuple: (u8, &str) = (0, "123");
    println!("{:?}", tuple);
    // TODO: vector example
    // using module
    let mut number:Vec<u8> = Vec::new();
    number.push(1);
    number.push(2);
    println!("{:?}", number);
    // using macro 
    let mut number_macro = vec![1];
    number_macro.push(3);
    println!("{:?}", number_macro);
    // TODO: hashmap example
    let mut fruit = HashMap::new();
    fruit.insert(3, "123");
    fruit.insert(4, "4456");
    let x = 3;
    println!("{:?}", fruit.get_key_value(&x));
    for (k,v) in &fruit {
        println!("{} {}", k, v);
    }
    println!("{:?}", fruit);
    // TODO: slice example

    let mut cars = vec!("Car one");
    cars.push("Car two");
    cars.push("Car three");

    //slice all [..]
    let a: &[&str] = &cars[..];
    println!("{:?}", a);
    // slice with index 0.1
    let temp: &[&str] = &cars[0..cars.len() - 1];
    println!("{:?}", temp);

    let mut demo = Vec::new();
    for i in 101..106 {
        demo.push(i.to_string());
    };   
    println!("{:?}", demo);
    let demo_1 = demo[0].clone();
    println!("{}", demo_1);

    let box_one = Box::new("123");
    println!("{:?}",box_one);
    
    let a = Dummy;
    let b = a;
    println!("{:?} {:?}",a,b);
   
    let fruit_a = Fruit { name: 123.to_string()};
    let fruit_copy = fruit_a.clone();
    println!("{:?}", fruit_a);
}
