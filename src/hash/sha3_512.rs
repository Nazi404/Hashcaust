use hex;
use sha3::{Digest, Sha3_512};

pub fn crack_sha3_512(hash: &str, word: &str) -> Option<String> {
    let mut hasher = Sha3_512::new();
    hasher.update(word);
    let digest = hasher.finalize();
    let hex_digest = hex::encode(digest);

    if hash == hex_digest {
        Some(word.to_string())
    } else {
        None
    }
}
