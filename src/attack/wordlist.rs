use std::fs::{File,read_to_string};
use std::io::{BufRead, BufReader};
use mlua::Lua;

pub fn wordlist(
    f: fn(&str, &str) -> Option<String>,
    path: &str,
    hash: &str,
    rule: Option<&String>,
) {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);

    if let Some(rule) = rule {

        let lua = Lua::new();
        let code = read_to_string("src/rules/rules.lua").unwrap();
        lua.load(&code).exec().unwrap();
        let globals = lua.globals();

        let rulefunc:mlua::Function = match rule.as_str() {
            "uppercase" => globals.get("uppercase").unwrap();
            "lowercase" => globals.get("lowercase").unwrap();
            "togglecase" => globals.get("togglecase").unwrap();
            "capitalize" => globals.get("capitalize").unwrap();
            "reverse" => globals.get("reverse").unwrap();
            "leet" => globals.get("leet").unwrap();
            _ => {
                eprintln!("Invalid rule");
                return;
            }
        }
    }
    else {}

    for line in reader.lines() {

        if let Ok(mut line) = line {

            if let Some(func) = &rulefunc {
                line = func.call((line)).unwrap();
            }

            if let Some(result) = f(hash, &line) {
                println!("[+] {}: {}", hash, result);
                break;
            }
        }
    }
}
