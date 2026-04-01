pub fn brute_mask(f: fn(&str, &str) -> Option<String>, mask: &str, hash: &str) {
    let charsets = parse_mask(mask);

    generate(String::new(), &charsets, 0, hash, f);
}

fn generate(
    prefix: String,
    charsets: &Vec<Vec<char>>,
    pos: usize,
    hash: &str,
    f: fn(&str, &str) -> Option<String>,
) {
    if pos == charsets.len() {
        if let Some(result) = f(hash, &prefix) {
            println!("[+] {}: {}", hash, result);
        }
        return;
    }

    for &c in &charsets[pos] {
        let mut new_prefix = prefix.clone();
        new_prefix.push(c);

        generate(new_prefix, charsets, pos + 1, hash, f);
    }
}

fn parse_mask(mask: &str) -> Vec<Vec<char>> {
    let mut charsets = Vec::new();
    let mut chars = mask.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '?' {
            if let Some(next) = chars.next() {
                let set = match next {
                    'l' => ('a'..='z').collect(),
                    'u' => ('A'..='Z').collect(),
                    'd' => ('0'..='9').collect(),
                    's' => "!@#$%^&*()_+-=[]{}|;:',.<>?/".chars().collect(),
                    'a' => (32u8..127).map(|c| c as char).collect(),
                    _ => vec![next],
                };
                charsets.push(set);
            }
        } else {
            // literal char
            charsets.push(vec![c]);
        }
    }

    charsets
}
