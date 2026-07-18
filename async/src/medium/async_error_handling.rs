/*
  Problem 93: Async Error Handling

  Write an async function fetch_data(id: u32) -> Result<String, String>.
  If id is 0, return Err("Invalid ID"). Otherwise, sleep for 5ms and return
  Ok("Data_<id>"). Use the ? operator in another async function
  fetch_and_process(id: u32) -> Result<usize, String> to call fetch_data
  and return the length of the string.

  Run the tests for this problem with:
    cargo test --test async_error_handling_test
*/

use std::time::Duration;

use tokio::time::sleep;

pub async fn fetch_data(id: u32) -> Result<String, String> {
    if id == 0 {
        return Err("Invalid ID".to_string());
    };
    sleep(Duration::from_millis(5)).await;
    Ok(format!("Data_{}", id))
}

pub async fn fetch_and_process(id: u32) -> Result<usize, String> {
    Ok(fetch_data(id).await?.len())
}
