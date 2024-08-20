fn main() {
    let mut mystr = "hello Rust".to_string();
    let partstr = &mystr[6..].to_string();
    mystr.push_str("aceans");
    println!("mystr == {} and partstr == {}", mystr, partstr);
}
