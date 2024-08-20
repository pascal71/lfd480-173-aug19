fn main() {
    let text1 = String::from("Hello, Rust!"); // Create a String variable
    let text2 = text1; // Move text1 to text2

    println!("{}", text2); // This will work, as text2 now owns the data

    // Attempt to use text1 after the move
    println!("{}", text1); // This will cause a compiler error
}
