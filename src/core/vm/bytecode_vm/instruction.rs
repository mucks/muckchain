#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    PushInt = 0xaa,
    PushBool = 0xab,
    PushByte = 0xac,
    Add = 0xad,
    Sub = 0xae,
    Get = 0xaf,
}

impl TryFrom<u8> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0xaa => Ok(Self::PushInt),
            0xab => Ok(Self::PushBool),
            0xac => Ok(Self::PushByte),
            0xad => Ok(Self::Add),
            0xae => Ok(Self::Sub),
            0xaf => Ok(Self::Get),
            _ => Err(anyhow::anyhow!("Invalid Instruction: {}", value)),
        }
    }
}
