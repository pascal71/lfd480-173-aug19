use std::fs::File;
use std::io;

fn main() {
    // Call the function and handle the potential error
    match open_non_existent_file() {
        Ok(()) => println!("File opened successfully."),
        Err(e) => println!("Failed to open file: {}", e),
    }
}

fn open_non_existent_file() -> Result<(), io::Error> {
    // Attempt to open a file that does not exist
    let _file = File::open("non_existent_file.txt")?;

    // If the file is opened successfully (which it won't be), return Ok(())
    Ok(())
}
