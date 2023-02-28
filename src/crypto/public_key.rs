use p256::ecdsa::VerifyingKey;
use serde::{Deserialize, Serialize};
use sha2::Digest;

use crate::core::Address;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct PublicKey {
    key: p256::PublicKey,
}

impl PublicKey {
    pub fn new(key: p256::PublicKey) -> Self {
        Self { key }
    }
    pub fn address(&self) -> Address {
        let mut sha = sha2::Sha256::new();
        sha.update(self.key.to_string().as_bytes());
        let b = sha.finalize();
        Address::from_bytes(b[b.len() - 20..].as_ref())
    }
    pub fn verifying_key(&self) -> VerifyingKey {
        VerifyingKey::from(&self.key)
    }
}
