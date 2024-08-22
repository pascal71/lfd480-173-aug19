// Define the List enum with variants for the end of the list (Nil) and elements (Cons)
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // Use the List enum to create a simple linked list of integers: 1 -> 2 -> 3
    let list = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3, Box::new(List::Nil))))));

    // Implement function to print list elements to console, demonstrating traversal
    print_list(&list);
}

// Function to traverse and print each element of the list
fn print_list(list: &List) {
    match list {
        List::Cons(value, next) => {
            println!("{}", value);
            print_list(next);
        },
        List::Nil => return,
    }
}
