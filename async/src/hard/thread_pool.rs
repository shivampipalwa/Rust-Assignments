/*
  Problem 78: Thread Pool — Simple Worker Thread

  Implement a simple ThreadPool that can execute closures. The pool should
  maintain a fixed number of worker threads and use a channel to send
  jobs to workers. Implement new(size: usize) and execute<F>(&self, f: F).

  Run the tests for this problem with:
    cargo test --test thread_pool_test
*/

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub sender: mpsc::Sender<Job>,
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let mut workers = Vec::new();
        let (tx, rx) = mpsc::channel();
        let rx_arc = Arc::new(Mutex::new(rx));
        for i in 0..size {
            let rx_arc = Arc::clone(&rx_arc);
            workers.push(Worker::new(i, rx_arc));
        }
        ThreadPool {
            workers,
            sender: tx,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

pub struct Worker {
    pub id: usize,
    pub thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let message = rx.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing");
                    job();
                }
                Err(_) => {
                    eprintln!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        Worker {
            id: id,
            thread: handle,
        }
    }
}
