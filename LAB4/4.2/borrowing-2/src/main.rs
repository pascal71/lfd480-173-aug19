fn main() {
    let mut name = String::from("Rust");
    greet(&mut name);
    println!("{}", name);
}

fn greet(name: &mut String) {
    name.push_str("acean");
    println!("Hello, {}!", name);
}
