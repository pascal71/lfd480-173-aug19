use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;

// Step 1: Define Custom Error Types
#[derive(Debug)]
enum AppError {
    DivisionByZero,
    ParseError(ParseFloatError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::DivisionByZero => write!(f, "Cannot divide by zero"),
            AppError::ParseError(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl Error for AppError {}

impl From<ParseFloatError> for AppError {
    fn from(err: ParseFloatError) -> AppError {
        AppError::ParseError(err)
    }
}

// Step 2: Implement Division Logic with Error Handling
fn divide_numbers(numerator: &str, denominator: &str) -> Result<f64, AppError> {
    let numerator: f64 = numerator.parse()?;
    let denominator: f64 = denominator.parse()?;

    if denominator == 0.0 {
        Err(AppError::DivisionByZero)
    } else {
        Ok(numerator / denominator)
    }
}

// Step 3: Main Function to Use the Division Logic
fn main() {
    let num = "10";
    let denom = "0"; // Try changing this to a non-zero value or something non-numeric

    match divide_numbers(num, denom) {
        Ok(result) => println!("The result is {}", result),
        Err(e) => match e {
            AppError::DivisionByZero => println!("Error: {}", e),
            AppError::ParseError(_) => println!("Please provide valid numbers. Error: {}", e),
        },
    }
}

