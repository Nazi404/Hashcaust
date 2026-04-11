use std::fs::{File,read_to_string};
use std::io::{BufRead, BufReader};
use mlua::Lua;

pub fn wordlist(
    f: fn(&str, &str) -> Option<String>,
    path: &str,
    hash: &str,
    rule: Option<&String>,
) {

    let mut found:bool = false;
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let lua = Lua::new();
    let mut rulefunc:Option<mlua::Function> = None;
    if let Some(rule) = rule {

        let code = read_to_string("src/rules/rules.lua").unwrap();
        lua.load(&code).exec().unwrap();
        let globals = lua.globals();

        rulefunc = Some(globals.get(rule.as_str()).expect("Unknown rule"));
    }

    for line in reader.lines() {

        if let Ok(mut line) = line {

            if let Some(func) = &rulefunc {
                line = func.call(line).unwrap();
            }

            if let Some(result) = f(hash, &line) {
                println!("[+] {}: {}", hash, result);
                found = true;
                break;
            }
        }
    }

    if !found {
        println!("[-] No match for this hash");
    }
}
