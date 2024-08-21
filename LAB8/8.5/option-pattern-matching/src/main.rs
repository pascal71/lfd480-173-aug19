fn main() {
    // Demonstrate Some Option using the function with both existing and non-existing keys
    let existing_key = "database_url";
    let non_existing_key = "unknown_key";

    match get_config_value(existing_key) {
        Some(value) => println!("Configuration for '{}': {}", existing_key, value),
        None => println!("No configuration found for key '{}'", existing_key),
    }

    match get_config_value(non_existing_key) {
        Some(value) => println!("Configuration for '{}': {}", non_existing_key, value),
        None => println!("No configuration found for key '{}'", non_existing_key),
    }
}

fn get_config_value(key: &str) -> Option<String> {
    // Predefined configuration values
    let config = vec![
        ("database_url", "postgres://localhost:5432"),
        ("api_key", "1234567890abcdef"),
        ("timeout", "30"),
    ];

    // Search for the key in the predefined configurations
    for &(config_key, config_value) in &config {
        if config_key == key {
            return Some(config_value.to_string());
        }
    }

    // Return None if the key is not found
    None
}
