use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Define a custom error type
#[derive(Debug)]
enum ThreadError {
    SimulatedError(String),
}

fn main() {
    // Create a channel to send results from threads
    let (tx, rx) = mpsc::channel();

    // Vector to hold the thread handles
    let mut handles = Vec::new();

    for i in 0..4 {
        // Spawn 4 threads
        let tx = tx.clone(); // Clone the transmitter for each thread

        let handle = thread::spawn(move || {
            println!("Thread {} is executing", i);

            // Simulate some work by sleeping the thread for 2 seconds
            thread::sleep(Duration::from_secs(2));

            // Simulate an error for one of the threads
            if i == 2 {
                let error =
                    ThreadError::SimulatedError(format!("Thread {} encountered an error", i));
                tx.send(Err(error)).unwrap();
            } else {
                // Send a success message back to the main thread
                tx.send(Ok(format!("Thread {} has finished executing", i)))
                    .unwrap();
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Process the results from the threads
    for received in rx.iter().take(4) {
        match received {
            Ok(message) => println!("{}", message),
            Err(e) => println!("Error: {:?}", e),
        }
    }

    println!("All threads have completed.");
}
