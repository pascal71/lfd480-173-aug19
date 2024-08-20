fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    // Create a String variable
    let my_string = String::from("Hello, Rust!");

    // Pass a reference to the String variable to calculate_length
    let length = calculate_length(&my_string);

    // Step 3: Use the original String variable again
    println!("The length of '{}' is {}.", my_string, length);

    // The original String can still be used here
    println!("The original string is still accessible: {}", my_string);
}

