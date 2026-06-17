/*
  Problem 6: Character Classifier

  Write a function that takes a char and returns a &'static str indicating whether it is
  "alphabetic", "numeric", "whitespace", or "other".

  Run the tests for this problem with:
    cargo test --test char_classifier_test
*/

pub fn classify_char(c: char) -> &'static str {
    match c {
        _ if c.is_alphabetic() => "alphabetic",
        _ if c.is_numeric() => "numeric",
        _ if c.is_whitespace() => "whitespace",
        _ => "other",
    }
}
