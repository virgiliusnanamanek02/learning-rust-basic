fn main() {
    // variable to print multiplication table for
    let i = 2;

    // counter variable that starts at 1
    let mut j = 1;

    // while loop that runs for 10 iterations
    while j <= 10 {
        // multiply i and j
        let mult = i * j;

        // print multiplication result on each iteration
        println!("{} * {} = {}", i, j, mult);

        // increase value of counter variable j
        j += 1;
    }
}
