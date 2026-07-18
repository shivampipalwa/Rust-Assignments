/*
  Problem 88: Message Passing — Oneshot Channel

  Write an async function oneshot_demo() that creates a tokio::sync::oneshot
  channel. Spawn a task that sends "done" through the channel. The main
  function should await the receiver and return the message.

  Run the tests for this problem with:
    cargo test --test oneshot_channel_test
*/

use tokio::{spawn, sync::oneshot};

pub async fn oneshot_demo() -> String {
    let (tx, rx) = oneshot::channel();
    spawn(async move {
        if let Err(_) = tx.send("done".to_string()) {
            println!("the receiver dropped");
        }
    });
    match rx.await {
        Ok(s) => {
            return s;
        }
        Err(_) => "the sender dropped".to_string(),
    }
}
