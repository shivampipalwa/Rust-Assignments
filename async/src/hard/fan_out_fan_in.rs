/*
  Problem 98: Async Fan-out / Fan-in

  Write an async function that takes a Vec<i32>, spawns a tokio task for each
  element to square it (fan-out), awaits all handles (fan-in), and returns
  the sum of the squared values.

  Run the tests for this problem with:
    cargo test --test fan_out_fan_in_test
*/

use tokio::spawn;

pub async fn fan_out_fan_in(v: Vec<i32>) -> i32 {
    let handles: Vec<_> = v.iter().map(|&x| spawn(async move { x * x })).collect();
    let mut ans = 0;
    for handle in handles {
        ans += handle.await.unwrap()
    }
    ans
}
