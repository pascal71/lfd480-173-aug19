fn main() {
    // Step 1: Declare and initialize a vector of i64 with numbers from 0 to 100
    let mut numbers: Vec<i64> = (0..=100).collect();

    // Step 2: Iterate over the elements in the vector and modify them
    for i in 0..numbers.len() {
        if numbers[i] % 2 == 0 {
            // Square the element's value if the original value is even
            numbers[i] = numbers[i] * numbers[i];
        } else {
            // Double the element's value if the original value is odd
            numbers[i] = numbers[i] * 2;
        }
    }

    // Step 3: Print out each value in the vector including their element number
    for (index, value) in numbers.iter().enumerate() {
        println!("Element {} -> {}", index, value);
    }
}
