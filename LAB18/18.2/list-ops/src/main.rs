// Define the List enum with variants for the end of the list (Nil) and elements (Cons)
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    // Append an element to the end of the list
    fn append(self, value: i32) -> List {
        match self {
            List::Cons(v, next) => List::Cons(v, Box::new(next.append(value))),
            List::Nil => List::Cons(value, Box::new(List::Nil)),
        }
    }

    // Remove the first occurrence of an element from the list
    fn remove(self, value: i32) -> List {
        match self {
            List::Cons(v, next) => {
                if v == value {
                    *next
                } else {
                    List::Cons(v, Box::new(next.remove(value)))
                }
            }
            List::Nil => List::Nil,
        }
    }

    // Find an element in the list
    fn find(&self, value: i32) -> bool {
        match self {
            List::Cons(v, next) => {
                if *v == value {
                    true
                } else {
                    next.find(value)
                }
            }
            List::Nil => false,
        }
    }
}

fn main() {
    // Create a linked list: 1 -> 2 -> 3
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    // Append an element to the list: 1 -> 2 -> 3 -> 4
    let list = list.append(4);
    println!("List after appending 4:");
    print_list(&list);

    // Remove an element from the list: 1 -> 3 -> 4
    let list = list.remove(2);
    println!("List after removing 2:");
    print_list(&list);

    // Find an element in the list
    let found = list.find(3);
    println!("Finding 3 in the list: {}", found);
}

// Function to traverse and print each element of the list
fn print_list(list: &List) {
    match list {
        List::Cons(value, next) => {
            println!("{}", value);
            print_list(next);
        }
        List::Nil => println!("End of list"),
    }
}
