use std::fmt::Display;

use crate::util::from_bytes;

pub struct Address([u8; 20]);

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl Address {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let address = from_bytes::<20>(bytes);
        Self(address)
    }
}
