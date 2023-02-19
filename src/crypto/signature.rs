use p256::ecdsa::signature::Verifier;
use serde::{Deserialize, Serialize};

use super::PublicKey;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Signature(pub p256::ecdsa::Signature);

impl Signature {
    pub fn verify(&self, data: &[u8], public_key: &PublicKey) -> bool {
        public_key.verifying_key().verify(data, &self.0).is_ok()
    }
}
