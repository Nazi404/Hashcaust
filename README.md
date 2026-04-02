# 🔐 HashCaust

«⚡ A fast, modular hash cracking tool written in Rust 🦀
Inspired by hashcat (CPU-based)»

---

🚀 Features

- 🔓 Multiple hash algorithm support
- ⚡ Wordlist attack mode
- 💥 Brute-force (mask-based) attack mode
- 🧠 Modular hash system
- 🎨 Colored CLI output
- 🖥️ Clean CLI interface (Clap आधारित)

---

📦 Supported Hash Types

argon2, bcrypt,
blake2b, blake2s, blake3,
md4, md5, ntlm,
sha1, sha224, sha256, sha384, sha512,
sha3_224, sha3_256, sha3_384, sha3_512

---

⚙️ Installation

1️⃣ Clone repo

git clone https://github.com/Anon-404/hashcaust.git
cd hashcaust

2️⃣ Build

cargo build --release

---

▶️ Usage

cargo run -- <hash> -t <type> -m <mode> [options]

---

🧠 Modes

📂 Wordlist Mode

cargo run -- <hash> -t <type> -m wordlist -w <wordlist>

✅ Example:

cargo run -- 5e884898da28047151d0e56f8dc6292773603d0d \
-t sha1 \
-m wordlist \
-w rockyou.txt

---

💥 Brute-force Mode (Mask Attack)

cargo run -- <hash> -t <type> -m brute -i <mask>

✅ Example:

cargo run -- <hash> -t md5 -m brute -i ?l?l?d?d

---

🎯 Mask Syntax

Pattern| Meaning| Example
"?l"| lowercase letters| a-z
"?u"| uppercase letters| A-Z
"?d"| digits| 0-9
"?s"| symbols| !@#$...
"?a"| all printable| full ASCII

Example:

?l?l?d?d

👉 Generates:

aa00 → zz99

---

🧠 How It Works

🔓 Wordlist Mode

1. Reads wordlist line-by-line
2. Applies selected hash function
3. Compares or verifies
4. Stops on match ✅

---

💥 Brute-force Mode

1. Parses mask ("?l?d?d" etc.)
2. Generates all combinations recursively
3. Tests each candidate
4. Prints match when found

---

📁 Project Structure

src/
 ├── main.rs
 ├── attack/
 │    ├── brute.rs
 │    └── wordlist.rs
 ├── hash/
 │    ├── md5.rs
 │    ├── ntlm.rs
 │    ├── bcrypt.rs
 │    ├── argon2.rs
 │    └── ...

---

⚠️ Important Notes

- ⚡ Fast hashes → direct compare (MD5, SHA, NTLM)
- 🐢 Slow hashes → verify (bcrypt, Argon2)
- 💀 Brute-force grows exponentially → use wisely

---

🔥 Future Improvements

- ⚡ Multi-threading (Rayon)
- 📊 Progress bar
- 🧠 Auto hash detection
- 🚀 Performance optimization
- 💻 Pre-built binary release

---

⚠️ Disclaimer

This tool is made for:

- 📚 Educational purposes
- 🔐 Security research

❌ Do NOT use for illegal activities

---

👨‍💻 Author

- GitHub: https://github.com/Anon-404

---

⭐ Support

If you like this project, give it a ⭐ on GitHub!
