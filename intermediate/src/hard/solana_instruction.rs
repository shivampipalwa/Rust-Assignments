/*
  Problem 60: Solana-Style Instruction — Unpack

  Simulate unpacking a Solana instruction. Define an enum Instruction with variants
  Initialize, Mint { amount: u64 }, and Transfer { amount: u64 }.
  Write a function unpack(data: &[u8]) -> Result<Instruction, String>.
  Data format: [tag: 1 byte][data: remaining bytes LE].
  Tags: 0 = Initialize, 1 = Mint, 2 = Transfer.

  Run the tests for this problem with:
    cargo test --test solana_instruction_test
*/

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Initialize,
    Mint { amount: u64 },
    Transfer { amount: u64 },
}

pub fn unpack(data: &[u8]) -> Result<Instruction, String> {
    if data.len() == 0 {
        return Err("instruction data is empty".to_string());
    }
    match data[0] {
        0 => return Ok(Instruction::Initialize),
        1 | 2 => {
            let amount_bytes: [u8; 8] = data[1..]
                .try_into()
                .map_err(|_| "Not enough bytes for amount".to_string())?;
            let amount = u64::from_le_bytes(amount_bytes);
            if data[0] == 1 {
                return Ok(Instruction::Mint { amount });
            }
            return Ok(Instruction::Transfer { amount });
        }
        _ => Err("Invalid Tag byte".to_string()),
    }
}
