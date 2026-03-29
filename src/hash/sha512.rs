use sha2::{Sha512, Digest};
use hex;

pub fn crack_sha512(hash:&str,word:&str) -> Option<String> {

    let mut hasher = Sha512::new();
    hasher.update(word);
    let digest = hasher.finalize();
    let hex_digest = hex::encode(digest);

    if hash == hex_digest {
        Some(word.to_string())
    } else {
        None
    }
}
