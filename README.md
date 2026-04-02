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
- 🖥️ Clean CLI (Clap powered)

---

## ⚙️ Installation

### 1️⃣ Install Rust

If Rust is not installed:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Reload shell:

```bash
source $HOME/.cargo/env
```

Verify installation:

```bash
rustc --version
cargo --version
```

---

### 2️⃣ Clone Repository

```bash
git clone https://github.com/Nazi404/Hashcaust.git
cd Hashcaust
```

---

### 3️⃣ Build Project

```bash
cargo build --release
```

👉 Binary location:

```bash
target/release/hashcaust
```

---

## ▶️ Usage

---

### 🔹 Run with Cargo (Development)

```bash
cargo run -- <hash> -t <type> -m <mode> [options]
```

---

### 🔹 Run with Binary

```bash
./target/release/hashcaust <hash> -t <type> -m <mode> [options]
```

---

## 🌍 Global Installation (Run from Anywhere)

### ✅ Option 1: Install to system PATH (recommended)

```bash
cp target/release/hashcaust /usr/local/bin/hashcaust
```

Now you can run:

```bash
hashcaust <hash> -t <type> -m <mode>
```

---

### ✅ Option 2: Install in user directory (no root)

```bash
mkdir -p ~/.local/bin
cp target/release/hashcaust ~/.local/bin/
```

Add to PATH:

```bash
echo 'export PATH=$HOME/.local/bin:$PATH' >> ~/.bashrc
source ~/.bashrc
```

Now run globally:

```bash
hashcaust <hash> -t <type> -m <mode>
```

---

## 🧠 Attack Modes

### 📂 Wordlist Mode

```bash
hashcaust <hash> -t <type> -m wordlist -w <wordlist>
```

#### Example:

```bash
hashcaust 5e884898da28047151d0e56f8dc6292773603d0d \
-t sha1 \
-m wordlist \
-w rockyou.txt
```

---

### 💥 Brute-force Mode (Mask Attack)

```bash
hashcaust <hash> -t <type> -m brute -i <mask>
```

#### Example:

```bash
hashcaust <hash> -t md5 -m brute -i ?l?l?d?d
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

---

## 📦 Supported Hash Types

### 🔓 Fast Hashes (Direct Compare)

- **MD Family**
  - md4
  - md5
  - ntlm (MD4-based)

- **SHA Family**
  - sha1
  - sha224
  - sha256
  - sha384
  - sha512

- **SHA-3 Family**
  - sha3_224
  - sha3_256
  - sha3_384
  - sha3_512

- **BLAKE Family**
  - blake2b
  - blake2s
  - blake3

### 🐢 Slow Hashes (Salted / Verify Required)

- argon2
- bcrypt

---

## ⚠️ Important Notes

- ⚡ Fast hashes → direct compare  
- 🐢 Slow hashes → require verification  
- 💀 Brute-force grows exponentially  

---

## ⚠️ Disclaimer

For educational & security research only.  
Do NOT use for illegal activities ❌

---

## 👨‍💻 Author

- https://github.com/Nazi404

---

## 📜 License

This project is licensed under the **GNU General Public License v3.0 (GPL-3.0)** – see the [LICENSE](LICENSE) file for details.

---

### 🔹 Summary

- ✅ Free to use, modify, and distribute  
- ✅ Copyleft: derivative works must also be GPL-3.0 licensed  
- ✅ No warranty or liability  
- ✅ Must include original copyright and license notice

--

## ⭐ Support

If you like this project, give it a ⭐ on GitHub!
