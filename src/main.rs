mod hash;
use hash::*;

fn main() {
    match crack_md5("604c8dd5066ee30539037569a028dc9b","William"){
        Some(hash) => println!("Hash found (md5): {}", hash),
        None => println!("Hash not found"),
    }

    match crack_sha1("943a702d06f34599aee1f8da8ef9f7296031d699","Hello, world!"){
        Some(hash) => println!("Hash found (sha1): {}", hash),
        None => println!("Hash not found"),
    }

    match crack_sha256("190eb3ebae2b41124493ac98ac49717fbd290156ca2e0de5f1d5c25a11e89120","William"){
        Some(hash) => println!("Hash found (sha256): {}", hash),
        None => println!("Hash not found"),
    }

    match crack_sha384("12f7f8e62880de9996913e7718701bd00f31e341f1e81e05b1ccf64379ce5cf5f841241f1a84aa8d9f08cbc1e276c3e4","William"){
        Some(hash) => println!("Hash found (sha382): {}", hash),
        None => println!("Hash not found"),
    }

    match crack_sha512("03fc33e5ab85d91dbd2fff8e844bb05d8b153cd5ddc2ae25cbf97ab26fac02664fd624b8ea59dabc0d0f0177da0b3a63244754983c7a9dd8264ee5057f689fd4","William"){
        Some(hash) => println!("Hash found (sha512): {}", hash),
        None => println!("Hash not found"),
    }

    match crack_blake2("553c8db366bb24504795f2afe4644b8312ad18bfe73e043a6a5a46310cd025c208dc34b7be52bc101f19164a23d1172f08dfa491956bc70744174f275d63b79f","William"){
        Some(hash) => println!("Hash found (blake2): {}", hash),
        None => println!("Hash not found"),
    }
}
