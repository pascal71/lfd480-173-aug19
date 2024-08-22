use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Define a struct for the Thread Pool
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// Type alias for the job that each worker will execute
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    // Create a new ThreadPool with a specified number of threads
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // Share the receiver among all workers
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        // Spawn the workers
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // Method to execute a job in the thread pool
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

// Define a struct for a Worker
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new Worker
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// Implement Drop for ThreadPool to gracefully shut down all workers
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

fn main() {
    // Create a thread pool with 4 workers
    let pool = ThreadPool::new(4);

    // Submit 8 jobs to the thread pool
    for i in 0..8 {
        pool.execute(move || {
            println!("Task {} is being processed", i);
            // Simulate some work
            thread::sleep(std::time::Duration::from_secs(2));
            println!("Task {} is completed", i);
        });
    }

    // The ThreadPool will be dropped at the end of main, and all workers will be shut down
}
