use std::io;

fn main() {
    println!("Enter a number: ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Convert the input string to an integer
    let my_nr: i32 = guess.trim().parse().expect("Please type a valid number!");

    // Check if the number is zero
    if my_nr == 0 {
        println!("The number is zero.");
    } else {
        // Check if the number is odd or even
        if my_nr % 2 == 0 {
            println!("The number {} is even.", my_nr);
        } else {
            println!("The number {} is odd.", my_nr);
        }
    }
}
