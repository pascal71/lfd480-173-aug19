struct Person {
    name: String,
    age: u32,
    email: String,
    active: bool,
}

fn main() {
    // Create an instance of `Person`
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
        active: true,
    };

    // Create a new instance of `Person` based on `person1`, updating some fields
    let person2 = Person {
        name: String::from("Bob"),
        age: 25,
        ..person1
    };

    println!(
        "Person 2: {}, {}, {}, {}",
        person2.name, person2.age, person2.email, person2.active
    );
}
