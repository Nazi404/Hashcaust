use hex;
use sha3::{Digest, Sha3_384};

pub fn crack_sha3_384(hash: &str, word: &str) -> Option<String> {
    let mut hasher = Sha3_384::new();
    hasher.update(word);
    let digest = hasher.finalize();
    let hex_digest = hex::encode(digest);

    if hash == hex_digest {
        Some(word.to_string())
    } else {
        None
    }
}
