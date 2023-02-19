use p256::ecdsa::{signature::Signer, SigningKey};

use super::{PublicKey, Signature};

#[derive(Clone, Debug)]
pub struct PrivateKey {
    key: p256::SecretKey,
}

impl PrivateKey {
    pub fn generate() -> Self {
        let key = p256::SecretKey::random(&mut rand::thread_rng());
        Self { key }
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey::new(self.key.public_key())
    }

    pub fn sign(&self, data: &[u8]) -> Signature {
        let signing_key = SigningKey::from(&self.key);
        let signature: p256::ecdsa::Signature = signing_key.sign(data);
        Signature(signature)
    }
}
