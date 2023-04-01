fn main() {
    let random_tuple = ("Hello", 200, 3.14);

    // accessing tuple element at index 0
    println!("Value at Index 0 = {}", random_tuple.0);

    // accessing tuple element at index 1
    println!("Value at Index 1 = {}", random_tuple.1);

    // accessing tuple element at index 2
    println!("Value at Index 2 = {}", random_tuple.2);

    let (msg, nums, pi) = random_tuple;

    println!("{}, {}, {}", msg, nums, pi);
}
