use bcrypt::verify;

pub fn crack_bcrypt(hash: &str, word: &str) -> Option<String> {
    if verify(word, hash).unwrap_or(false) {
        Some(word.to_string())
    } else {
        None
    }
}
