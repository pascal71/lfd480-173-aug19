use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    // Vector to hold the thread handles
    let mut handles = Vec::new();

    for i in 0..4 { // Spawn 4 threads
        let tx = tx.clone(); // Clone the transmitter for each thread

        let handle = thread::spawn(move || {
            println!("Thread {} is executing", i);
            // Simulate some work by sleeping the thread for 2 seconds
            thread::sleep(Duration::from_secs(2));

            // Send a message back to the main thread
            tx.send(format!("Thread {} has finished executing", i)).unwrap();
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Receive and print messages from the threads
    for received in rx.iter().take(4) {
        println!("{}", received);
    }

    println!("All threads have completed.");
}
