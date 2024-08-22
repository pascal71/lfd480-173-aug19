fn starts_with_a<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
    if s1.starts_with('a') {
        s1
    } else if s2.starts_with('a') {
        s1 // Note: We still need to return an &'a str, so we return s1 if s2 starts with 'a'
    } else {
        s1
    }
}

fn main() {
    let string1 = "example";
    let string2 = "sample";
    let result = starts_with_a(string1, string2);
    println!("Result: {}", result);
}
