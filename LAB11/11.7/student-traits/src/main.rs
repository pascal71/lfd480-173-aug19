use std::time::{SystemTime, UNIX_EPOCH};

// Define the Student struct
pub struct Student {
    pub firstname: String,
    pub lastname: String,
    pub timestamp: u64,
    pub languages: Vec<String>,
}

impl Student {
    // Constructor to create a new Student
    pub fn new(firstname: &str, lastname: &str, languages: Vec<String>) -> Self {
        Student {
            firstname: firstname.to_string(),
            lastname: lastname.to_string(),
            timestamp: Self::current_timestamp(),
            languages,
        }
    }

    // Function to get the current timestamp
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

// Implement the Clone trait for Student with custom timestamp update
impl Clone for Student {
    fn clone(&self) -> Self {
        Student {
            firstname: self.firstname.clone(),
            lastname: self.lastname.clone(),
            timestamp: Student::current_timestamp(), // Update the timestamp on cloning
            languages: self.languages.clone(),
        }
    }
}

fn main() {
    let languages = vec!["Rust".to_string(), "Python".to_string(), "Java".to_string()];
    let student1 = Student::new("John", "Doe", languages);

    // Simulate some time passing
    println!("Processing...");
    std::thread::sleep(std::time::Duration::from_secs(2));

    let student2 = student1.clone();

    println!(
        "Original Student: {} {}, Timestamp: {}, Languages: {:?}",
        student1.firstname, student1.lastname, student1.timestamp, student1.languages
    );

    println!(
        "Cloned Student: {} {}, Timestamp: {}, Languages: {:?}",
        student2.firstname, student2.lastname, student2.timestamp, student2.languages
    );
}
