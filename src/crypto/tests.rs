use super::*;

#[test]
fn test_keypair_sign_verify_valid() {
    let private_key = PrivateKey::generate();
    let public_key = private_key.public_key();

    let msg = b"hello world";
    let sig = private_key.sign(msg);

    assert!(sig.verify(msg, &public_key));
}

#[test]
fn test_keypair_sign_verify_fail() {
    let private_key = PrivateKey::generate();
    let _public_key = private_key.public_key();

    let msg = b"hello world";
    let sig = private_key.sign(msg);

    let other_private_key = PrivateKey::generate();
    let other_public_key = other_private_key.public_key();

    assert!(!sig.verify(msg, &other_public_key));
    assert!(!sig.verify(b"wrong message", &other_public_key));
}
