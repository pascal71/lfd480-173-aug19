fn consume_vector(v: Vec<String>) {
    for element in v {
        println!("{}", element);
    }
    // The vector `v` goes out of scope here and is dropped.
}

fn main() {
    // Create a vector of Strings
    let my_vector = vec![
        String::from("Hello"),
        String::from("World"),
        String::from("Rust"),
    ];

    // Pass the vector to consume_vector, transferring ownership
    consume_vector(my_vector);

    // Step 3: Attempt to use the original vector after it has been moved
    // Uncommenting the line below will cause a compiler error
    // println!("{:?}", my_vector);
}
