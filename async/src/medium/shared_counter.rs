/*
  Problem 74: Shared State — Mutex Counter

  Write a function that spawns 10 threads. Each thread should increment a shared
  counter 100 times. Use Arc<Mutex<i32>> to share the counter safely.
  Wait for all threads to finish and return the final counter value.

  Run the tests for this problem with:
    cargo test --test shared_counter_test
*/

use std::sync::{Arc, Mutex};
use std::thread;

pub fn multithreaded_counter() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let counter_clone = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..100 {
                    let mut lock = counter_clone.lock().unwrap();
                    *lock += 1;
                }
            })
        })
        .collect();
    for handle in handles {
        let _ = handle.join();
    }
    let lock = counter.lock().unwrap();
    *lock
}
