fn main() {
    let text1 = String::from("Hello, Rust!"); // Step 1
    let text2 = text1; // Step 2

    // Step 3: Trying to use text1 after it has been moved to text2
    println!("{}", text1); // This will cause a compiler error
}
