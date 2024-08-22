// Define the greeting function
fn greeting(salutation: &str, name: &str) -> String {
    format!("{} {}", salutation, name)
}

fn main() {
    // Create a closure that curries the greeting function with a default salutation
    let quickhi = |name: &str| greeting("Hasta la vista", name);

    // Use the closure to greet someone
    println!("{}", quickhi("John"));
    println!("{}", quickhi("Alice"));
}
