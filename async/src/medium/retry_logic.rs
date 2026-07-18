/*
  Problem 92: Async Retry Logic

  Write an async function retry_operation<F, Fut, T, E>(f: F, max_retries: usize)
  -> Result<T, E> where F is a factory that produces a future Fut.
  If the future returns an Err, retry up to max_retries times before giving up.
  Wait 10ms between retries.

  Run the tests for this problem with:
    cargo test --test retry_logic_test
*/

use std::future::Future;
use tokio::time::{sleep, Duration};

pub async fn retry_operation<F, Fut, T, E>(mut f: F, mut max_retries: usize) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut retries = 0;
    loop {
        match f().await {
            Ok(val) => {
                return Ok(val);
            }
            Err(e) => {
                if retries >= max_retries {
                    return Err(e);
                }
                retries += 1;
                sleep(Duration::from_millis(10)).await;
            }
        }
    }
}
