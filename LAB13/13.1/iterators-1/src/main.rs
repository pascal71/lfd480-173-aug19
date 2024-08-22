fn main() {
    // Step 1: Create a vector with the given numbers
    let numbers = vec![-4, 2, 3, 1, -7, 21, 42, 31, -7];

    // Step 2: Print all the numbers using an iterator
    println!("Original numbers:");
    for num in &numbers {
        println!("{}", num);
    }

    println!(); // Adding a blank line for separation

    // Step 3: Using an adapter iterator and a closure to modify the numbers
    let modified_numbers: Vec<f64> = numbers
        .iter()
        .map(|&x| {
            if x % 2 == 0 {
                x as f64 * 2.0 // Double even numbers
            } else {
                x as f64 / 2.0 // Half odd numbers
            }
        })
        .collect();

    // Print the modified numbers
    println!("Modified numbers (double even, half odd):");
    for num in &modified_numbers {
        println!("{}", num);
    }

    println!(); // Adding a blank line for separation

    // Step 4: Filter out numbers divisible by 5
    let filtered_numbers: Vec<f64> = modified_numbers
        .into_iter()
        .filter(|&x| x % 5.0 != 0.0)
        .collect();

    // Print the filtered numbers
    println!("Filtered numbers (not divisible by 5):");
    for num in &filtered_numbers {
        println!("{}", num);
    }
}
