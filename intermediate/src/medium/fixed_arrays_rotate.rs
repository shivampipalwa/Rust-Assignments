/*
  Problem 51: Fixed-Size Arrays — Rotate Left

  Write a function that takes a fixed-size array [u8; 8] and a rotation count usize,
  and returns a new [u8; 8] with elements rotated left by that many positions.
  Rotation wraps around.

  Run the tests for this problem with:
    cargo test --test fixed_arrays_rotate_test
*/

fn reverse_slice(arr: &mut [u8]) {
    let n = arr.len();
    for i in 0..n / 2 {
        let x = arr[i];
        arr[i] = arr[n - i - 1];
        arr[n - i - 1] = x;
    }
}

pub fn rotate_left(arr: [u8; 8], count: usize) -> [u8; 8] {
    let mut brr = arr.clone();
    reverse_slice(&mut brr[0..count % 8]);
    reverse_slice(&mut brr[count % 8..]);
    reverse_slice(&mut brr);
    return brr;
}
