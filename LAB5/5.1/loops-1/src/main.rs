fn main() {
    for number in 0..=100 {
        if number % 3 == 0 || number % 5 == 0 {
            println!("{}", number);
        }
    }
}
