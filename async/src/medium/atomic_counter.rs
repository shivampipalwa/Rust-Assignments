/*
  Problem 79: Atomic Counter

  Rewrite the multithreaded counter problem (Problem 74) using AtomicI32
  instead of Mutex<i32>. Compare the performance and complexity.
  Show use of fetch_add and Ordering.

  Run the tests for this problem with:
    cargo test --test atomic_counter_test
*/

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;

pub fn atomic_counter() -> i32 {
    let counter = Arc::new(AtomicI32::new(0));
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let counter_clone = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..100 {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    for handle in handles {
        let _ = handle.join();
    }
    counter.load(Ordering::Relaxed)
}
