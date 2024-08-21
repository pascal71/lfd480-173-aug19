enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn str_to_operation(s: &str) -> Option<Operation> {
    match s.to_lowercase().as_str() {
        "add" => Some(Operation::Add),
        "subtract" => Some(Operation::Subtract),
        "multiply" => Some(Operation::Multiply),
        "divide" => Some(Operation::Divide),
        _ => None,
    }
}

fn calculate(op: Operation, a: f64, b: f64) -> f64 {
    match op {
        Operation::Add => a + b,
        Operation::Subtract => a - b,
        Operation::Multiply => a * b,
        Operation::Divide => a / b,
    }
}

fn main() {
    let operation_str = "divide";
    let x = 5.0;
    let y = 3.0;

    match str_to_operation(operation_str) {
        Some(op) => {
            let result = calculate(op, x, y);
            println!("Result of {}ing {} and {}: {}", operation_str, x, y, result);
        }
        None => println!("Unknown operation: {}", operation_str),
    }
}
