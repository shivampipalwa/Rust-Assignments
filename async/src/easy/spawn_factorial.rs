/*
  Problem 84: Async Join — Spawn Factorial

  Write an async function that spawns a tokio task (tokio::spawn) to compute
   the factorial of 5 (120). Return the JoinHandle's result after awaiting it.

  Run the tests for this problem with:
    cargo test --test spawn_factorial_test
*/

use tokio::spawn;

pub async fn spawn_factorial() -> u64 {
    let fact_handle = spawn(async {
        let mut fact = 1;
        for i in 1..=5 {
            fact *= i;
        }
        fact
    });
    fact_handle.await.unwrap()
}
