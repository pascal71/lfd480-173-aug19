fn main() {
    // Test with a valid age
    check_age(25);

    // Test with an invalid age to trigger a panic
    check_age(-5);
}

fn check_age(age: i32) {
    if age < 0 {
        panic!("Age cannot be negative: {}", age);
    } else {
        println!("Valid age: {}", age);
    }
}
