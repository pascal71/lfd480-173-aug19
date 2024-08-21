use std::fs::File;
use std::io;
use std::io::Read;

// Function to read a file's contents into a String
fn read_file_to_string(filename: &str) -> Result<String, io::Error> {
    // Attempt to open the file
    let mut file = File::open(filename)?;

    // String to hold the file contents
    let mut contents = String::new();

    // Attempt to read the file's contents into the string
    file.read_to_string(&mut contents)?;

    // Return the contents if successful
    Ok(contents)
}

fn main() -> Result<(), io::Error> {
    // Call the function with the filename
    let filename = "example.txt";

    // Read the file contents and handle any potential errors
    let contents = read_file_to_string(filename)?;

    // If successful, print the contents to the console
    println!("File contents:\n{}", contents);

    // Return Ok(()) to indicate successful execution
    Ok(())
}
