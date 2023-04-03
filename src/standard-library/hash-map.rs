// import HashMap from Rust standard collections library
use std::collections::HashMap;

fn main() {
    // create a new HashMap
    let mut fruits: HashMap<i32, String> = HashMap::new();

    // add key-value in a hashmap
    println!("Insert new value");
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));

    println!("fruits = {:?}", fruits);

    // access values in a hashmap
    println!("Access value");
    let first_fruit = fruits.get(&1);
    let second_fruit = fruits.get(&2);
    let third_fruit = fruits.get(&3);

    println!("first fruit = {:?}", first_fruit);
    println!("second fruit = {:?}", second_fruit);
    println!("third fruit = {:?}", third_fruit);

    // remove value in a hashmap
    fruits.remove(&1);

    println!("fruits after remove operation = {:?}", fruits);
}
