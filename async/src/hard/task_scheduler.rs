/*
  Problem 100: Async Task Scheduler

  Implement a TaskScheduler that can schedule async closures to run after
  a specific delay. It should use a PriorityQueue (or sorted Vec) to keep track
  of tasks and a background tokio task to execute them when their time comes.

  Run the tests for this problem with:
    cargo test --test task_scheduler_test
*/

use std::{
    collections::BinaryHeap,
    sync::{Arc, Mutex},
};
use tokio::{
    spawn,
    time::{sleep, Duration, Instant},
};

// Struct to hold scheduled task with the time to execute it
pub struct ScheduledTask {
    pub execute_at: Instant,
    pub task: Box<dyn FnOnce() + Send + 'static>,
}

// Trait implementations for ScheduledTask for BinaryHeap
// ScheduledTask with smaller(earlier) execute_at will have be higher in priority
impl PartialEq for ScheduledTask {
    fn eq(&self, other: &Self) -> bool {
        self.execute_at == other.execute_at
    }
}

impl Eq for ScheduledTask {}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.execute_at.cmp(&self.execute_at)
    }
}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct TaskScheduler {
    pub tasks: Arc<Mutex<BinaryHeap<ScheduledTask>>>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        TaskScheduler {
            tasks: Arc::new(Mutex::new(BinaryHeap::new())),
        }
    }

    pub fn schedule<F>(&self, delay: Duration, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let execution_time = Instant::checked_add(&Instant::now(), delay).unwrap();
        let mut lock = self.tasks.lock().unwrap();
        lock.push(ScheduledTask {
            execute_at: execution_time,
            task: Box::new(f),
        });
    }

    // Spawn a background task to process queue
    // It can include infinite loop
    pub fn start(&self) {
        let tasks = self.tasks.clone();
        spawn(async move {
            loop {
                let now = Instant::now();
                // Check if the next task is ready WITHOUT holding the lock for long
                let task_to_run = {
                    let mut lock = tasks.lock().unwrap();
                    if let Some(task) = lock.peek() {
                        if task.execute_at <= now {
                            Some(lock.pop().unwrap())
                        } else {
                            None
                        }
                    } else {
                        None // Queue is empty
                    }
                }; // Lock dropped

                // Execute or Wait
                if let Some(scheduled_task) = task_to_run {
                    spawn(async move {
                        (scheduled_task.task)();
                    });
                } else {
                    sleep(Duration::from_millis(10)).await;
                }
            }
        });
    }
}
