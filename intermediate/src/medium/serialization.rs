/*
  Problem 47: Serialization — Manual to_bytes / from_bytes

  Define a struct Record { id: u32, value: u16 }. Implement methods
  to_bytes(&self) -> Vec<u8> and from_bytes(data: &[u8]) -> Result<Self, String>
  using little-endian byte order. The serialized format should be
  [id: 4 bytes][value: 2 bytes] = 6 bytes total.

  Run the tests for this problem with:
    cargo test --test serialization_test
*/

#[derive(Debug, PartialEq)]
pub struct Record {
    pub id: u32,
    pub value: u16,
}

impl Record {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(6);

        data.extend_from_slice(&self.id.to_le_bytes());
        data.extend_from_slice(&self.value.to_le_bytes());

        data
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() != 6 {
            return Err(format!("Expected exactly 6 bytes, but got {}", data.len()));
        }

        let id_bytes: [u8; 4] = data[0..4]
            .try_into()
            .map_err(|_| "Failed to extract id bytes")?;

        let value_bytes: [u8; 2] = data[4..6]
            .try_into()
            .map_err(|_| "Failed to extract value bytes")?;

        Ok(Record {
            id: u32::from_le_bytes(id_bytes),
            value: u16::from_le_bytes(value_bytes),
        })
    }
}
