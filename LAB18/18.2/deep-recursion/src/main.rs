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
    // Create a large linked list with 100,000 elements
    let mut list = List::Nil;
    for i in 0..100_000 {
        list = list.append(i);
    }

    println!("List created with 100,000 elements.");
    // Attempt to find an element deep in the list
    let found = list.find(99_999);
    println!("Finding element 99,999: {}", found);
}
