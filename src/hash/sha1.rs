use hex;
use sha1::{Digest, Sha1};

pub fn crack_sha1(hash: &str, word: &str) -> Option<String> {
    let mut hasher = Sha1::new();
    hasher.update(word);
    let digest = hasher.finalize();
    let hex_digest = hex::encode(digest);

    if hash == hex_digest {
        Some(word.to_string())
    } else {
        None
    }
}
