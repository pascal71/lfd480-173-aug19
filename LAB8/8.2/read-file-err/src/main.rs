use std::fs::File;
use std::io::Read;

fn main() {
    let result = read_file_content("rust.txt");

    match result {
        Ok(content) => {
            println!("File content read successfully.");
            println!("Content: \n{}", content);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn open_file(filename: &str) -> Result<File, String> {
    match File::open(filename) {
        Ok(file) => Ok(file),
        Err(e) => Err(e.to_string()),
    }
}

fn read_file_content(filename: &str) -> Result<String, String> {
    let mut file = match open_file(filename) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => {
            if content.contains("Ferris") {
                Ok(content)
            } else {
                Err(String::from("The string does not contain 'Ferris'"))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
