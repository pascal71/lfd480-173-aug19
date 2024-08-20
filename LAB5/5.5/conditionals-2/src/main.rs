use std::io;

fn main() {
    println!("Enter a number: ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Convert the input string to an integer
    let my_nr: i32 = guess.trim().parse().expect("Please type a valid number!");

    // Use match to check the value of the number
    match my_nr {
        0 => println!("The number is zero."),
        n if n % 2 == 0 => println!("The number {} is even.", n),
        n if n % 2 != 0 => println!("The number {} is odd.", n),
        _ => println!("Unexpected case."),
    }
}
