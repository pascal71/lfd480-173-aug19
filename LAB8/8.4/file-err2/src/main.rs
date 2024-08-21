use std::fs::File;
use std::io::{self, Write};

fn create_and_write_file(filename: &str, content: &str) -> Result<(), io::Error> {
    // Attempt to create the file
    let mut file = File::create(filename)?;

    // Attempt to write the content to the file
    file.write_all(content.as_bytes())?;

    // If everything succeeds, return Ok
    Ok(())
}

fn main() {
    // Define the filename and content
    let filename = "output.txt";
    let content = "Hello, Rust!";

    // Call the function and handle the result
    match create_and_write_file(filename, content) {
        Ok(()) => {
            println!("File '{}' created and written successfully.", filename);
        }
        Err(e) => {
            println!(
                "Failed to create and write to file '{}'. Error: {}",
                filename, e
            );
        }
    }
}
