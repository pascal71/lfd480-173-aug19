use std::thread;
use std::time::Duration;

fn main() {
    // Vector to hold the thread handles
    let mut handles = Vec::new();

    for i in 0..4 {
        // Spawn 4 threads
        let handle = thread::spawn(move || {
            println!("Thread {} is executing", i);
            // Simulate some work by sleeping the thread for 2 seconds
            thread::sleep(Duration::from_secs(2));
            println!("Thread {} has finished executing", i);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads have completed.");
}
