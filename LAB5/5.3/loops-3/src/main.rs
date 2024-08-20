fn main() {
    // Define the Unicode string
    let text = "♣ Dia dhuit, an Domhan! ♣";

    // Iterate over each character in the string
    for c in text.chars() {
        // Print the character followed by a space, without a newline
        print!("{} ", c);
    }
    // Print a newline at the end for formatting purposes
    println!();
}
