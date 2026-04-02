# 🔐 HashCaust

> ⚡ A fast, modular hash cracking tool written in **Rust** 🦀  
> Inspired by **hashcat** (CPU-based, no GPU support)

---

## 🚀 Features

- 🔓 Supports multiple hash algorithms  
- ⚡ Wordlist-based attack mode  
- 💥 Mask-based brute-force attack  
- 🧠 Modular and extensible architecture  
- 🎨 Colored CLI output  
- 🖥️ Clean and flexible CLI (powered by Clap)

---

## 📦 Supported Hash Types

```
argon2, bcrypt,
blake2b, blake2s, blake3,
md4, md5, ntlm,
sha1, sha224, sha256, sha384, sha512,
sha3_224, sha3_256, sha3_384, sha3_512
```

---

## ⚙️ Installation

### 1️⃣ Clone the Repository

```bash
git clone https://github.com/Anon-404/hashcaust.git
cd hashcaust
```

### 2️⃣ Build the Project

```bash
cargo build --release
```

---

## ▶️ Usage

```bash
cargo run -- <hash> -t <type> -m <mode> [options]
```

---

## 🧠 Attack Modes

### 📂 Wordlist Mode

```bash
cargo run -- <hash> -t <type> -m wordlist -w <wordlist>
```

#### ✅ Example

```bash
cargo run -- 5e884898da28047151d0e56f8dc6292773603d0d \
-t sha1 \
-m wordlist \
-w rockyou.txt
```

---

### 💥 Brute-force Mode (Mask Attack)

```bash
cargo run -- <hash> -t <type> -m brute -i <mask>
```

#### ✅ Example

```bash
cargo run -- <hash> -t md5 -m brute -i ?l?l?d?d
```

---

## 🎯 Mask Syntax

| Pattern | Description          | Characters        |
|--------|---------------------|------------------|
| `?l`   | Lowercase letters   | a-z              |
| `?u`   | Uppercase letters   | A-Z              |
| `?d`   | Digits              | 0-9              |
| `?s`   | Symbols             | !@#$%^&*...      |
| `?a`   | All printable ASCII | Full range       |

#### Example Mask

```
?l?l?d?d
```

👉 Generates:
```
aa00 → zz99
```

---

## 🧠 How It Works

### 🔓 Wordlist Mode

1. Loads target hash  
2. Reads wordlist line-by-line  
3. Applies selected hash function  
4. Compares or verifies result  
5. Stops when match is found ✅  

---

### 💥 Brute-force Mode

1. Parses mask (`?l?d?d` etc.)  
2. Generates all combinations recursively  
3. Tests each candidate  
4. Prints match when found  

---

## 📁 Project Structure

```
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
```

---

## ⚠️ Important Notes

- ⚡ Fast hashes → direct comparison (MD5, SHA, NTLM)  
- 🐢 Slow hashes → require verification (bcrypt, Argon2)  
- 💀 Brute-force grows exponentially → use wisely  

---

## 🔥 Roadmap / Future Improvements

- ⚡ Multi-threading (Rayon)
- 📊 Real-time progress bar
- 🧠 Automatic hash detection
- 🚀 Performance optimization
- 💻 Pre-built binaries (release builds)
- 🎯 Hashcat-style modes (`-m 0`, `-m 1000`)

---

## ⚠️ Disclaimer

This tool is intended for:

- 📚 Educational purposes  
- 🔐 Security research  

❌ Do **NOT** use this tool for illegal activities.

---

## 👨‍💻 Author

- GitHub: https://github.com/Anon-404

---

## ⭐ Support

If you like this project, consider giving it a ⭐ on GitHub!

---

## 💬 Final Note

> "Fast hashes fall quickly. Slow hashes test your patience." 😏
