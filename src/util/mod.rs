mod random;

pub fn from_bytes<const N: usize>(bytes: &[u8]) -> [u8; N] {
    if bytes.len() != N {
        panic!(
            "given bytes with length {} is not a valid hash, it must be {N} bytes long",
            bytes.len()
        );
    }
    let mut hash = [0; N];
    hash.copy_from_slice(bytes);
    hash
}
