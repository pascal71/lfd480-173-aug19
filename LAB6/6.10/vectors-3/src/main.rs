// Define the Book struct
struct Book {
    title: String,
    author: String,
    stock: u32,
}

// Define the Inventory struct
struct Inventory {
    books: Vec<Book>,
}

impl Inventory {
    // Function to add a book to the inventory
    fn add_book(&mut self, title: String, author: String, stock: u32) {
        let book = Book {
            title,
            author,
            stock,
        };
        self.books.push(book);
    }

    // Function to check the stock of a specific book by title
    fn check_stock(&self, title: &str) -> Option<u32> {
        for book in &self.books {
            if book.title == title {
                return Some(book.stock);
            }
        }
        None
    }

    // Function to list the entire inventory
    fn list_inventory(&self) {
        for book in &self.books {
            println!(
                "Title: {}, Author: {}, Stock: {}",
                book.title, book.author, book.stock
            );
        }
    }

    // Function to sell a book (decrease its stock by 1)
    fn sell_book(&mut self, title: &str) -> bool {
        for book in &mut self.books {
            if book.title == title {
                if book.stock > 0 {
                    book.stock -= 1;
                    println!(
                        "Sold one copy of '{}'. Remaining stock: {}",
                        book.title, book.stock
                    );
                    return true;
                } else {
                    println!("'{}' is out of stock.", book.title);
                    return false;
                }
            }
        }
        println!("'{}' not found in inventory.", title);
        false
    }
}

fn main() {
    // Initialize the inventory
    let mut inventory = Inventory { books: Vec::new() };

    // Add some books to the inventory
    inventory.add_book(
        String::from("The Rust Programming Language"),
        String::from("Steve Klabnik and Carol Nichols"),
        10,
    );
    inventory.add_book(
        String::from("Programming Rust"),
        String::from("Jim Blandy and Jason Orendorff"),
        5,
    );
    inventory.add_book(
        String::from("Rust in Action"),
        String::from("Tim McNamara"),
        7,
    );

    // List the entire inventory
    println!("Current inventory:");
    inventory.list_inventory();

    // Check stock for a specific book
    let title = "Programming Rust";
    match inventory.check_stock(title) {
        Some(stock) => println!("Stock for '{}': {}", title, stock),
        None => println!("'{}' not found in inventory.", title),
    }

    // Sell a book
    inventory.sell_book("Programming Rust");
    inventory.sell_book("Programming Rust");
    inventory.sell_book("Nonexistent Book");

    // List inventory after selling
    println!("\nInventory after sales:");
    inventory.list_inventory();
}

