use hex;
use md4::{Md4, Digest};

pub fn crack_ntlm(hash: &str, word: &str) -> Option<String> {

    let mut utf16le: Vec<u8> = Vec::new();
    for c in word.encode_utf16() {
        utf16le.extend(&c.to_le_bytes());
    }

    let mut hasher = Md4::new();
    hasher.update(&utf16le);
    let digest = hasher.finalize();

    let hex_digest = hex::encode(digest);

    if hash == hex_digest {
        Some(word.to_string())
    } else {
        None
    }

}
