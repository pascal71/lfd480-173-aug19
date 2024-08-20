fn append_exclamation(s: &mut String) {
    s.push_str("!");
}

fn main() {
    // Create a mutable String variable
    let mut my_string = String::from("Hello, Rust");

    // Pass a mutable reference to the String variable to append_exclamation
    append_exclamation(&mut my_string);

    // Step 3: Print the modified string after calling append_exclamation
    println!("Modified string: {}", my_string);

    // Step 4: Try creating a second mutable borrow in main before the first borrow's scope ends
    // Uncomment the following lines to see the compiler error:
    let second_borrow = &mut my_string;
    append_exclamation(second_borrow);
    println!("Second borrow: {}", second_borrow);
}
