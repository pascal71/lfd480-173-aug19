use std::f64;

fn main() {
    // Test cases
    let number1 = 25.0;
    let number2 = -4.0;

    // Call the square root function and handle the result
    match square_root(number1) {
        Ok(result) => println!("The square root of {} is {}", number1, result),
        Err(e) => println!("Error: {}", e),
    }

    match square_root(number2) {
        Ok(result) => println!("The square root of {} is {}", number2, result),
        Err(e) => println!("Error: {}", e),
    }
}

// Step 1: Write a function that returns the square root of an f64
fn square_root(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        // Step 2: Refuse to calculate the square root if x < 0
        return Err(String::from(
            "Cannot calculate the square root of a negative number.",
        ));
    }
    Ok(x.sqrt())
}

