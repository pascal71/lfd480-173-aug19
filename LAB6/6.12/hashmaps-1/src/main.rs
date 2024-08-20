use std::collections::HashMap;

fn main() {
    // Step 1: Declare and initialize the hashmap with student names and programming languages
    let mut devstudents: HashMap<String, String> = HashMap::new();

    // Populate the hashmap with the given records
    devstudents.insert(String::from("Fred"), String::from("Golang"));
    devstudents.insert(String::from("Alice"), String::from("Perl"));
    devstudents.insert(String::from("Kjell"), String::from("Python"));
    devstudents.insert(String::from("Jarmo"), String::from("Rust"));
    devstudents.insert(String::from("Sander"), String::from("Zig"));

    // Step 2: Print out all the elements in the hashmap
    println!("Initial student programming languages:");
    for (student, language) in &devstudents {
        println!("{}: {}", student, language);
    }

    // Step 3: Add a new student and language (with a check)
    let new_student = String::from("Eve");
    let new_language = String::from("JavaScript");

    if devstudents.contains_key(&new_student) {
        println!("{} is already in the hashmap.", new_student);
    } else {
        devstudents.insert(new_student.clone(), new_language.clone());
        println!("Added {} with language {}", new_student, new_language);
    }

    // Step 4: Change Sander's programming language to Rust
    if let Some(language) = devstudents.get_mut("Sander") {
        *language = String::from("Rust");
        println!("Updated Sander's language to Rust.");
    }

    // Step 5 (Extra Credit): Convert all programming languages to uppercase
    for (_, language) in devstudents.iter_mut() {
        *language = language.to_uppercase();
    }

    // Print out the updated hashmap
    println!("\nUpdated student programming languages:");
    for (student, language) in &devstudents {
        println!("{}: {}", student, language);
    }
}

