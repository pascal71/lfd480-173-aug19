fn main() {
    let name = String::from("Rust");
    greet(&name);
    println!("{}", name);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}
