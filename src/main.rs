use clap::{command, Arg};

mod attack;
mod hash;

use attack::*;
use hash::*;

fn main() {
    let match_cli = command!()
        .arg_required_else_help(true)
        .arg(Arg::new("hash").help("Target hash").required(true))
        .arg(
            Arg::new("wordlist")
                .short('w')
                .long("wordlist")
                .value_name("FILE")
                .help("Wordlist file"),
        )
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .help("Hash type")
                .required(true),
        )
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .help("Attack mode")
                .required(true),
        )
        .get_matches();

    let hash = match_cli.get_one::<String>("hash").unwrap();
    let wordlst = match_cli.get_one::<String>("wordlist").unwrap();
    let type_hash = match_cli.get_one::<String>("type").unwrap();
    let mode = match_cli.get_one::<String>("mode").unwrap();

    println!("Hash: {}", hash);
    println!("Wordlist: {}", wordlst);
    println!("Type hash: {}", type_hash);
    println!("Mode: {}", mode);

    match mode.as_str() {
        "wordlist" => match type_hash.as_str() {
            "md5" => {
                wordlist(crack_md5, wordlst.as_str(), hash.as_str());
            }
            "sha1" => {
                wordlist(crack_sha1, wordlst.as_str(), hash.as_str());
            }
            "sha256" => {
                wordlist(crack_sha256, wordlst.as_str(), hash.as_str());
            }
            "sha384" => {
                wordlist(crack_sha384, wordlst.as_str(), hash.as_str());
            }
            "sha512" => {
                wordlist(crack_sha512, wordlst.as_str(), hash.as_str());
            }
            "blake2" => {
                wordlist(crack_blake2, wordlst.as_str(), hash.as_str());
            }
            _ => {
                eprintln!("Hash type not supported or invalid type");
                return;
            }
        },
        "brute" => {
            println!("Brute mode coming soon");
        }
        _ => {
            eprintln!("❌ Invalid mode!");
            return;
        }
    }
}
