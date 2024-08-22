// Define the struct with a lifetime annotation
struct Person<'a> {
    name: &'a str,
}

// Implement the make_person function with the same lifetime annotation
fn make_person<'a>(name: &'a str) -> Person<'a> {
    Person { name }
}

fn main() {
    let name = "Alice";
    let alice = make_person(name);
    println!("Name: {}", alice.name);
}
