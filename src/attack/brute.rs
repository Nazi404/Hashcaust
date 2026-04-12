use mlua::{Function, Lua};
use std::fs::read_to_string;

pub fn brute_mask(
    f: fn(&str, &str) -> Option<String>,
    mask: &str,
    hash: &str,
    rule: Option<&String>,
) {
    let charsets = parse_mask(mask);
    let lua = Lua::new();
    let mut rulefunc: Option<Function> = None;

    if let Some(rule) = rule {
        let code = read_to_string("src/rules/rules.lua").expect("Failed to read rules.lua");
        lua.load(&code).exec().expect("Lua load failed");
        let globals = lua.globals();
        rulefunc = Some(globals.get(rule.as_str()).expect("Unknown rule name"));
    }
    generate(String::new(), &charsets, 0, hash, f, &rulefunc);
}

fn generate(
    prefix: String,
    charsets: &Vec<Vec<char>>,
    pos: usize,
    hash: &str,
    f: fn(&str, &str) -> Option<String>,
    rulefunc: &Option<Function>,
) {
    if pos == charsets.len() {
        let mut candidate = prefix;
        if let Some(func) = rulefunc {
            match func.call::<String>(candidate.clone()) {
                Ok(result) => candidate = result,
                Err(_) => return,
            }
        }

        if let Some(result) = f(hash, &candidate) {
            println!("[+] {}: {}", hash, result);
        }

        return;
    }

    for &c in &charsets[pos] {
        let mut new_prefix = prefix.clone();
        new_prefix.push(c);

        generate(new_prefix, charsets, pos + 1, hash, f, rulefunc);
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
            charsets.push(vec![c]);
        }
    }
    charsets
}
