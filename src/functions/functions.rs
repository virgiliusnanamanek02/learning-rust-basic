fn addsub(a: i32, b: i32) -> (i32, i32) {
    return (a + b, a - b);
}

fn main() {
    let (sum, diff) = addsub(4, 1);
    println!("Sum = {}, Difference = {}", sum, diff);
}
