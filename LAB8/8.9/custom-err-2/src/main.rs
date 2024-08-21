use std::error::Error;
use std::fmt;

// Step 1: Define the MyError enum with different error variants
#[derive(Debug)]
enum MyError {
    NotFound,
    PermissionDenied,
    ConnectionFailed,
    Timeout,
}

// Step 2: Implement the Error trait for MyError
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::NotFound => write!(f, "Resource not found"),
            MyError::PermissionDenied => write!(f, "Permission denied"),
            MyError::ConnectionFailed => write!(f, "Connection failed"),
            MyError::Timeout => write!(f, "Operation timed out"),
        }
    }
}

impl Error for MyError {}

// Step 3: Write a function that returns Result<(), MyError>
fn perform_operation(op_type: u8) -> Result<(), MyError> {
    match op_type {
        1 => Err(MyError::NotFound),
        2 => Err(MyError::PermissionDenied),
        3 => Err(MyError::ConnectionFailed),
        4 => Err(MyError::Timeout),
        _ => Ok(()), // If the operation type is not recognized, we assume success
    }
}

// Step 4: Handle the different error cases in main
fn main() {
    let operation_type = 2; // Change this to 1, 2, 3, or 4 to simulate different errors

    match perform_operation(operation_type) {
        Ok(()) => println!("Operation completed successfully."),
        Err(e) => match e {
            MyError::NotFound => println!("Error: {}", e),
            MyError::PermissionDenied => println!("Error: {}", e),
            MyError::ConnectionFailed => println!("Error: {}", e),
            MyError::Timeout => println!("Error: {}", e),
        },
    }
}

