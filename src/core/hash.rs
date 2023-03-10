use core::fmt;

use crate::util::from_bytes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let hash = from_bytes::<32>(bytes);
        Self(hash)
    }
    pub fn zero() -> Self {
        Self([0; 32])
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}
