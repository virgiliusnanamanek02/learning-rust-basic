fn main() {
    let grade: i32 = 92;

    if grade >= 90 && grade <= 100 {
        println!("Congratulations!, your grade is A");
    } else if grade >= 75 && grade < 90 {
        println!("Great!, your grade is B");
    } else if grade >= 60 && grade < 75 {
        println!("Cool!, your grade is C");
    } else if grade >= 45 && grade < 60 {
        println!("Hmmm!, your grade is D");
    } else if grade >= 0 && grade < 45 {
        println!("Sorry!, your grade is E");
    } else {
        println!("Your are not graded");
    }
}
