use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn wordlist(f: fn(&str, &str) -> Option<String>, path: &str, hash: &str) {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(result) = f(hash, &line) {
                println!("[+] {}: {}", hash, result);
                break;
            }
        }
    }
}
