use std::sync::{Arc, RwLock};
use std::thread;

// Define a struct for the Counter with a RwLock for thread-safe access
struct Counter {
    count: RwLock<i32>,
}

impl Counter {
    // Function to create a new Counter
    fn new() -> Self {
        Counter {
            count: RwLock::new(0),
        }
    }

    // Function to increment the counter
    fn increment(&self) {
        let mut count = self.count.write().unwrap();
        *count += 1;
    }

    // Function to get the current value of the counter
    fn get_count(&self) -> i32 {
        let count = self.count.read().unwrap();
        *count
    }
}

fn main() {
    let counter = Arc::new(Counter::new());

    // Create threads to increment the counter
    let mut handles = vec![];
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            counter_clone.increment();
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of the counter
    println!("Final counter value: {}", counter.get_count());
}
