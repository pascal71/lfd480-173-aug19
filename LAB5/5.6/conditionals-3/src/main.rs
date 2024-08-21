// Define an enum named Operation with variants for each of the 4 operations.
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// Implement a function that converts a string into the Operation enum.
fn str_to_operation(s: &str) -> Operation {
    match s.to_lowercase().as_str() {
        "add" => Operation::Add,
        "subtract" => Operation::Subtract,
        "multiply" => Operation::Multiply,
        "divide" => Operation::Divide,
        _ => {
            println!("Unknown operation: {}. Defaulting to addition.", s);
            Operation::Add
        }
    }
}

// Implement the calculator using a match expression to choose the appropriate arithmetic operation based on the enum variant.
fn calculate(op: Operation, a: f64, b: f64) -> f64 {
    match op {
        Operation::Add => a + b,
        Operation::Subtract => a - b,
        Operation::Multiply => a * b,
        Operation::Divide => {
            if b != 0.0 {
                a / b
            } else {
                println!("Cannot divide by zero! Defaulting to 0.");
                0.0
            }
        }
    }
}

// Test the calculator in the main function with a few examples.
fn main() {
    let operation_str = "divide"; // You can change this to "subtract", "multiply", "divide", or an invalid operation
    let x = 3.0;
    let y = 0.0;

    let op = str_to_operation(operation_str);
    let result = calculate(op, x, y);
    println!("Result of {}ing {} and {}: {}", operation_str, x, y, result);
}
