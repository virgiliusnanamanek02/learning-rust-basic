fn italic() {
    let i = 6;
    println!("i- {}", i);
}

fn bold() {
    let a = 5;
    let b = 100;
    let c = 1;
    println!("a- {} \nb- {} \nc- {}", a, b, c);
    italic();
}

fn main() {
    let x = 42;

    println!("x- {}", x);

    bold();
}
