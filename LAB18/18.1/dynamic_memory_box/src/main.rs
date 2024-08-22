fn main() {
    // Creating a large vector with 1 million elements
    let large_vector: Box<Vec<i32>> = Box::new(vec![0; 1_000_000]);
    println!(
        "Successfully created a large vector with {} elements on the heap",
        large_vector.len()
    );

    // Demonstrating ownership transfer
    let another_location = take_ownership(large_vector);
    println!(
        "The large vector now lives in a new function: {:?}",
        another_location.len()
    );
}

fn take_ownership(data: Box<Vec<i32>>) -> Box<Vec<i32>> {
    // Function takes ownership of the Box, simulating a transfer of ownership
    data
}
