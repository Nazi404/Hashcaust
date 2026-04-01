use hex;
use md4::{Digest, Md4};

pub fn crack_md4(hash: &str, word: &str) -> Option<String> {
    let mut hasher = Md4::new();
    hasher.update(word.as_bytes());
    let result = hasher.finalize();
    let digest = hex::encode(result);

    if hash == digest {
        Some(word.to_string())
    } else {
        None
    }
}
