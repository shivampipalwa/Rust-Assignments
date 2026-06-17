/*
  Problem 9: Borrow and Count

  Write a function that takes an immutable reference to a Vec<i32> and returns the count of elements greater than a given threshold.
  This exercises borrowing without taking ownership.

  Run the tests for this problem with:
    cargo test --test borrow_count_test
*/

pub fn count_above(values: &Vec<i32>, threshold: i32) -> usize {
    values
        .iter()
        .fold(0, |acc, x| if (*x > threshold) { acc + 1 } else { acc })
}
