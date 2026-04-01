use md5;

pub fn crack_md5(hash: &str, word: &str) -> Option<String> {
    println!("{:?}", word);
    let digest = format!("{:x}", md5::compute(word));
    if hash == digest {
        Some(word.to_string())
    } else {
        None
    }
}
