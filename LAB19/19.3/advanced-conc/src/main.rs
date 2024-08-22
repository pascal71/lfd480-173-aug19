use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel with a transmitter (tx) and a receiver (rx)
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx)); // Wrap the receiver in Arc and Mutex for safe sharing across threads

    // Create multiple producer threads
    for i in 0..5 {
        // 5 producers
        let tx_clone = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let message = format!("Message from producer {}", i);
            // Simulate some work
            thread::sleep(Duration::from_secs(1));
            tx_clone.send(message).unwrap();
            println!("Producer {} sent its message", i);
        });
    }

    // Create multiple consumer threads
    let num_consumers = 3;
    let mut handles = vec![];
    for i in 0..num_consumers {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            while let Ok(message) = rx_clone.lock().unwrap().recv() {
                println!("Consumer {} received: {}", i, message);
                // Simulate some processing time
                thread::sleep(Duration::from_secs(2));
            }
        });
        handles.push(handle);
    }

    // Wait for all consumers to complete
    for handle in handles {
        handle.join().unwrap();
    }
}
