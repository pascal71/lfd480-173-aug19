fn main() {
    // Step 1: Create a vector containing the words "hello", "world", and "rust"
    let words = vec!["hello", "world", "rust"];

    // Step 2: Convert the vector of words into a single string where words are separated by a space
    let sentence = words.join(" ");

    // Step 3: Print this string
    println!("{}", sentence);
}
