use solana_program::{program_error::ProgramError};
use std::convert::TryInto;

pub enum HelloWorldInstruction {
    Increment,
    Decrement,
    Set(u32)
}

impl HelloWorldInstruction {
    pub fn unpack(data: &[u8]) -> Result<HelloWorldInstruction, ProgramError> {
        let (&first, rest) = data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        match first {
            0 => Ok(HelloWorldInstruction::Increment),
            1 => Ok(HelloWorldInstruction::Decrement),
            2 => {
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let mut arr: [u8; 4] = [0,0,0,0];
                let mut i = 0;
                for ele in rest {
                    arr[i] = ele.to_owned();
                    i += 1;
                }

                let value = u32::from_le_bytes(arr);

                Ok(HelloWorldInstruction::Set(value))
            },
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
}