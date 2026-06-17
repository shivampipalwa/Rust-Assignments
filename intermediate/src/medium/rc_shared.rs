/*
  Problem 40: Rc<T> — Shared Ownership

  Create a function that takes a String value, wraps it in Rc<String>,
  clones the Rc twice, and returns a tuple of the strong reference count
  and the string value itself (cloned for the return).
  This exercises shared ownership without copying the underlying data.

  Run the tests for this problem with:
    cargo test --test rc_shared_test
*/

use std::rc::Rc;

pub fn shared_ownership(value: String) -> (usize, String) {
    let rc_val = Rc::new(value);
    let _ = Rc::clone(&rc_val);
    let clone2 = Rc::clone(&rc_val);
    return (Rc::strong_count(&clone2), (*rc_val).clone());
}
