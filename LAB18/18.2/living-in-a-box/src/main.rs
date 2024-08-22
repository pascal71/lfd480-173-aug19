use std::ops::Deref;

// Define a custom smart pointer MyBox
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implement Deref to allow MyBox to be dereferenced like a regular reference
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

enum List {
    Cons(i32, MyBox<List>),
    Nil,
}

impl List {
    fn append(self, value: i32) -> List {
        match self {
            List::Cons(v, next) => List::Cons(v, MyBox::new(next.append(value))),
            List::Nil => List::Cons(value, MyBox::new(List::Nil)),
        }
    }
}

fn main() {
    let list = List::Cons(
        1,
        MyBox::new(List::Cons(
            2,
            MyBox::new(List::Cons(3, MyBox::new(List::Nil))),
        )),
    );
    let list = list.append(4);
    print_list(&list);
}

fn print_list(list: &List) {
    match list {
        List::Cons(value, next) => {
            println!("{}", value);
            print_list(next);
        }
        List::Nil => println!("End of list"),
    }
}
