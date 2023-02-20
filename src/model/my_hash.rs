use crate::util::from_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MyHash([u8; 32]);

impl MyHash {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let hash = from_bytes::<32>(bytes);
        Self(hash)
    }
}
