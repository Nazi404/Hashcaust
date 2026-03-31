use blake2::{Blake2s256, Digest};
use hex;

pub fn crack_blake2s(hash: &str, word: &str) -> Option<String> {
    let mut hasher = Blake2s256::new();
    hasher.update(word.as_bytes());
    let digest = hasher.finalize();
    let hex_digest = hex::encode(digest);

    if hash == hex_digest {
        Some(word.to_string())
    } else {
        None
    }
}
