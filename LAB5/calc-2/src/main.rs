use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
enum OperationParseError {
    InvalidOperation(String),
}

impl fmt::Display for OperationParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationParseError::InvalidOperation(op) => write!(f, "Invalid operation: {}", op),
        }
    }
}

impl std::error::Error for OperationParseError {}

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl FromStr for Operation {
    type Err = OperationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "add" => Ok(Operation::Add),
            "subtract" => Ok(Operation::Subtract),
            "multiply" => Ok(Operation::Multiply),
            "divide" => Ok(Operation::Divide),
            _ => Err(OperationParseError::InvalidOperation(s.to_string())),
        }
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
    let operation_str = "dividem";
    let x = 5.0;
    let y = 3.0;

    let op: Result<Operation, _> = operation_str.trim().to_lowercase().parse();

    match op {
        Ok(op) => {
            let result = calculate(op, x, y);
            println!("Result of {}ing {} and {}: {}", operation_str, x, y, result);
        }
        Err(e) => println!("Error: {}", e),
    }
}
