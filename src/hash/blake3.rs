use blake3;

pub fn crack_blake3(hash: &str, word: &str) -> Option<String> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(word.as_bytes());
    let digest = hasher.finalize();
    let hex_digest = digest.to_hex();

    if hash.eq_ignore_ascii_case(&hex_digest) {
        Some(word.to_string())
    } else {
        None
    }
}
