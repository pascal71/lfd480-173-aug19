// Define the Fibonacci struct
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    // Create a new instance of Fibonacci iterator
    pub fn new() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // Calculate the next Fibonacci number
        let current_fib = self.curr;
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(current_fib)
    }
}

fn main() {
    println!(); // Adding a blank line for separation

    // Testing Fibonacci iterator
    let mut fibonacci = Fibonacci::new();
    println!("First 10 Fibonacci numbers:");
    for i in 1..=10 {
        if let Some(v) = fibonacci.next() {
            println!("Fibonacci {} = {}", i, v);
        }
    }
}
