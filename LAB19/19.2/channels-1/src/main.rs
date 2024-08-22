use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel with a transmitter (tx) and a receiver (rx)
    let (tx, rx) = mpsc::channel();

    // Spawn producer threads
    for i in 0..4 {
        let tx_clone = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let message = format!("Message from thread {}", i);
            // Simulate some work
            thread::sleep(Duration::from_secs(1));
            tx_clone.send(message).unwrap();
            println!("Thread {} has sent its message", i);
        });
    }

    // Consume messages in the main thread
    for _ in 0..4 {
        let received = rx.recv().unwrap();
        println!("Main thread received: {}", received);
    }
}
