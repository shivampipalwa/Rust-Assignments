/*
  Problem 77: Message Passing — Data Pipeline

  Create a three-stage pipeline using mpsc channels:
  1. Producer: Sends numbers 1..=5.
  2. Processor: Receives numbers, squares them, and sends to next stage.
  3. Consumer: Receives squared numbers and sums them.
  Implement this using three threads and return the final sum.

  Run the tests for this problem with:
    cargo test --test pipeline_test
*/

use std::sync::mpsc;
use std::thread;

pub fn data_pipeline() -> i32 {
    let (tx1, rx1) = mpsc::channel::<i32>();
    let (tx2, rx2) = mpsc::channel::<i32>();
    thread::spawn(move || {
        for i in 1..=5 {
            tx1.send(i).unwrap();
        }
    });
    thread::spawn(move || {
        for x in rx1 {
            tx2.send(x * x).unwrap();
        }
    });
    let ans = thread::spawn(move || {
        let mut ans = 0;
        for x in rx2 {
            ans += x;
        }
        ans
    })
    .join()
    .unwrap();
    ans
}
