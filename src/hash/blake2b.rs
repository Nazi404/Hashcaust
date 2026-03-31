use blake2::{Blake2b512, Digest};
use hex;

pub fn crack_blake2b(hash: &str, word: &str) -> Option<String> {
    let mut hasher = Blake2b512::new();
    hasher.update(word.as_bytes());
    let digest = hasher.finalize();
    let hex_digest = hex::encode(digest);

    if hash == hex_digest {
        Some(word.to_string())
    } else {
        None
    }
}
