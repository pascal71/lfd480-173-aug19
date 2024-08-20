// Step 1: Define a Person struct
struct Person {
    first_name: String,
    last_name: String,
    age: u32,
}

fn main() {
    // Step 2: Instantiate the struct in main
    let person = Person {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        age: 30,
    };

    // Step 3: Print the contents of the struct
    println!("First Name: {}", person.first_name);
    println!("Last Name: {}", person.last_name);
    println!("Age: {}", person.age);

    // Step 4: Print "Adult" if the person has reached the adult age
    if person.age >= 18 {
        println!("Adult");
    } else {
        println!("Not an adult");
    }
}
