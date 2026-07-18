/*
  Problem 96: Async Task Queue

  Implement an async TaskQueue where you can push async closures and have a
  fixed number of workers process them. Use a mpsc channel for the queue.
  Workers should be spawned as tokio tasks.

  Run the tests for this problem with:
    cargo test --test async_task_queue_test
*/

use std::sync::Arc;
use tokio::{
    spawn,
    sync::{mpsc, Mutex},
};

pub struct TaskQueue {
    pub sender: mpsc::Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl TaskQueue {
    pub fn new(worker_count: usize) -> Self {
        let (tx, rx) = mpsc::channel::<Box<dyn FnOnce() + Send + 'static>>(100);
        let rx_arc = Arc::new(Mutex::new(rx));
        for _ in 0..worker_count {
            let rx = Arc::clone(&rx_arc);
            spawn(async move {
                loop {
                    let job_result = {
                        let mut lock = rx.lock().await;
                        lock.recv().await
                    };
                    match job_result {
                        Some(job) => job(),
                        None => break,
                    }
                }
            });
        }
        TaskQueue { sender: tx }
    }

    pub async fn push<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).await.unwrap();
    }
}
