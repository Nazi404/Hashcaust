use argon2::{Argon2, PasswordHash, PasswordVerifier};

pub fn crack_argon2(hash: &str, word: &str) -> Option<String> {
    let parsed_hash = PasswordHash::new(hash).ok()?;
    let argon2 = Argon2::default();

    if argon2.verify_password(word.as_bytes(), &parsed_hash).is_ok() {
        Some(word.to_string())
    } else {
        None
    }
}
