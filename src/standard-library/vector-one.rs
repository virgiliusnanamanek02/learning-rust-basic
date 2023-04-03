fn main() {
    // vector creation with vec! macro
    let mut numbers = vec![1, 2, 3];

    println!("v2= {:?}", numbers);

    //add new value
    numbers.push(4);
    numbers.push(5);

    //remove a value

    numbers.remove(1);

    println!("v2= {:?}", numbers);

    let colors = vec!["blue", "red", "green"];
    // method 1: access vector elements using vector index
    println!("first color = {}", colors[0]);
    println!("second color = {}", colors[1]);
    println!("third color = {}", colors[2]);

    // method 2: access vector elements using get() method and vector index
    println!("first color = {:?}", colors.get(0));
    println!("second color = {:?}", colors.get(1));
    println!("third color = {:?}", colors.get(2));

    for index in 0..3 {
        println!("Index: {} -- Value: {}", index, colors[index]);
    }

    let mut v: Vec<i32> = Vec::new();

    // push values to a mutable vector
    v.push(10);
    v.push(20);

    println!("v = {:?}", v);
}
