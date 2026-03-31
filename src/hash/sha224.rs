use hex;
use sha2::{Digest, Sha224};

pub fn crack_sha224(hash: &str, word: &str) -> Option<String> {
    let mut hasher = Sha224::new();
    hasher.update(word);
    let digest = hasher.finalize();
    let hex_digest = hex::encode(digest);

    if hash == hex_digest {
        Some(word.to_string())
    } else {
        None
    }
}
