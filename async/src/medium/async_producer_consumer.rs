/*
  Problem 87: Async Producer-Consumer (Tokio Channel)

  Rewrite Problem 76 using tokio::sync::mpsc. Spawn a producer task that sends
  ten integers. The main function should concurrently receive them and return
  the results in a Vec.

  Run the tests for this problem with:
    cargo test --test async_producer_consumer_test
*/

use tokio::sync::mpsc;

pub async fn async_producer_consumer() -> Vec<i32> {
    let (tx, mut rx) = mpsc::channel(1);
    tokio::spawn(async move {
        for i in 1..=10 {
            tx.send(i).await.unwrap();
        }
    });
    let mut arr = Vec::new();
    while let Some(i) = rx.recv().await {
        arr.push(i);
    }
    arr
}
