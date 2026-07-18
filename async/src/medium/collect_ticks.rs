/*
  Problem 95: Async Streams — Collect Ticks

  Write an async function that creates a tokio::time::interval (1ms).
  Collect the first 5 "ticks" (Instant values) from the interval stream
  into a Vec and return it.

  Run the tests for this problem with:
    cargo test --test collect_ticks_test
*/

use tokio::time::{interval, Duration, Instant};

pub async fn collect_ticks() -> Vec<Instant> {
    let mut interval = interval(Duration::from_millis(1));
    let mut vec = vec![];
    for _ in 0..5 {
        let int = interval.tick().await;
        vec.push(int);
    }
    // let vec: Vec<Instant> = (0..5).map(async |_| interval.tick().await).collect();
    vec
}
