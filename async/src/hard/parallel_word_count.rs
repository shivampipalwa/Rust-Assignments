/*
  Problem 81: Parallel Word Count

  Write a function that takes a Vec<String> (lines of text) and counts the
  occurrences of words in parallel. Split the lines among 4 threads. Each thread
  computes a local HashMap, and then the main thread merges them into a final
  HashMap<String, usize>.

  Run the tests for this problem with:
    cargo test --test parallel_word_count_test
*/

use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub fn parallel_word_count(lines: Vec<String>) -> HashMap<String, usize> {
    let mut handles = vec![];
    let n = lines.len();
    let lines = Arc::new(lines);
    for i in 0..4 {
        let lines = Arc::clone(&lines);
        handles.push(thread::spawn(move || {
            let start = n * i / 4;
            let end = if i == 3 { n } else { n * (i + 1) / 4 };
            let mut count = HashMap::new();
            for line in &lines[start..end] {
                for word in line.split_whitespace() {
                    *count.entry(word.to_string()).or_insert(0) += 1;
                }
            }
            count
        }));
    }
    let mut counts = HashMap::new();
    for handle in handles {
        let map = handle.join().unwrap();
        for (word, count) in map {
            *counts.entry(word).or_insert(0) += count;
        }
    }
    counts
}
