use sha1::{Sha1, Digest};
use hex;

pub fn crack_sha1(hash:&str,word:&str) -> Option<String> {

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
