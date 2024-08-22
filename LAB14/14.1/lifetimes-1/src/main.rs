// Define the struct with lifetime annotations
struct AuthorAndExcerpt<'a> {
    author: &'a str,
    excerpt: &'a str,
}

impl<'a> AuthorAndExcerpt<'a> {
    // Implement the function to create a new AuthorAndExcerpt instance
    fn new_author_and_excerpt(author: &'a str, excerpt: &'a str) -> AuthorAndExcerpt<'a> {
        AuthorAndExcerpt { author, excerpt }
    }
}

fn main() {
    let name = "Hemingway";
    let text = "The world breaks everyone, and afterward, some are strong at the broken places.";

    // Create a new AuthorAndExcerpt instance
    let record = AuthorAndExcerpt::new_author_and_excerpt(name, text);

    // Optionally, print the result to verify
    println!("Author: {}, Excerpt: {}", record.author, record.excerpt);
}
