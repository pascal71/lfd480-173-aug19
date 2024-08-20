fn main() {
    // Define a string with all the letters of the alphabet
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    // Loop through each character in the string
    for letter in alphabet.chars() {
        // Check if the character is a vowel
        if "aeiou".contains(letter) {
            println!("{}", letter);
        }
    }
}
