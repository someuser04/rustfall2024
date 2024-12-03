use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared counter
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    // Spawning 5 threads
    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter); // Clone the Arc for each thread
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                // Lock the Mutex to safely increment the counter
                let mut num = counter_clone.lock().expect("Failed to lock the mutex");
                *num += 1;
            }
        });
        handles.push(handle);
    }

    // Joining all threads
    for handle in handles {
        handle.join().expect("Thread failed to join");
    }

    // Printing the final value of the counter
    println!("Final counter value: {}", *counter.lock().unwrap());
}
