struct LargeData {
    name: String,
    data: Box<Vec<i32>>,
}

fn main() {
    let large_data = LargeData {
        name: String::from("Large Vector Data"),
        data: Box::new(vec![0; 1_000_000]),
    };
    println!(
        "Created struct with large vector: {} with {} elements on the heap",
        large_data.name,
        large_data.data.len()
    );

    let moved_data = transfer_ownership(large_data);
    println!("Ownership of struct has been transferred. The data now lives in another function.");
    println!("New location data length: {:?}", moved_data.data.len());
}

fn transfer_ownership(data: LargeData) -> LargeData {
    // Transfer ownership of the struct
    data
}
