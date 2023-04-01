fn main() {
    // scope of outer_var variable is inside the main function code block
    let outer_var = 100;

    // start of the inner code block
    {
        // scope of inner_var variable is only inside this new code block
        let inner_var = 200;
        println!("inner_var = {}", inner_var);
        println!("outer_var inside inner block = {}", outer_var);
    }
    // end of the inner code block

    println!("outer_var = {}", outer_var);
}
