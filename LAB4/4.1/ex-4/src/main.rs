fn main() {
    let mut text4 = String::from("Hello, Rust!"); // Create a mutable String variable

    text4.push_str(" Welcome to mutable ownership."); // Append text to text4

    println!("{}", text4); // Print text4 after modification
}
