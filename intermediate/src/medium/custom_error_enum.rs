/*
  Problem 55: Custom Error Enum (thiserror-style)

  Define an error enum DataError with variants InvalidLength { expected, actual },
  ChecksumMismatch, and Utf8Error(std::string::FromUtf8Error).
  Implement std::fmt::Display and std::error::Error manually.
  Write a function validate_packet that checks a &[u8] has exactly 10 bytes
  with a valid checksum (last byte = XOR of all previous bytes).

  Run the tests for this problem with:
    cargo test --test custom_error_enum_test
*/

use std::fmt;

#[derive(Debug)]
pub enum DataError {
    InvalidLength { expected: usize, actual: usize },
    ChecksumMismatch,
    Utf8Error(std::string::FromUtf8Error),
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::InvalidLength { expected, actual } => write!(
                f,
                "Invalid length. expected: {}, actual: {}",
                expected, actual
            ),
            DataError::ChecksumMismatch => write!(f, "Checksum mismatch."),
            DataError::Utf8Error(e) => write!(f, "UTF8 error. {e}"),
        }
    }
}

impl std::error::Error for DataError {}

pub fn validate_packet(data: &[u8]) -> Result<(), DataError> {
    if data.len() != 10 {
        return Err(DataError::InvalidLength {
            expected: 10,
            actual: data.len(),
        });
    }
    let mut xor = 0;
    for (i, byte) in data.iter().enumerate() {
        if i == data.len() - 1 {
            break;
        }
        xor ^= byte;
    }
    if *data.last().unwrap() != xor {
        return Err(DataError::ChecksumMismatch);
    }
    Ok(())
}
