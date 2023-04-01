fn main() {
    // define a multi-line closure
    let squared_sum = |x: i32, y: i32| {
        // find the sum of two parameters
        let sum: i32 = x + y;

        // find the squared value of the sum
        let result: i32 = sum * sum;

        return result;
    };

    // call the closure
    let result = squared_sum(5, 3);

    println!("Result = {}", result);
}
