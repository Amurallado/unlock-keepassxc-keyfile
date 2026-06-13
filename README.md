# 🔐 kplock – Secure CLI to Unlock KeePassXC with Encrypted Keyfile

**kplock** is a Rust‑based tool that unlocks KeePassXC using an encrypted keyfile encrypted with **age**. The workflow is fully automated and guarantees that the decrypted keyfile exists only while KeePassXC is running, securely wiping it afterwards.

## 🚀 Features
- **Encryption with age** (scrypt + X25519) – no external dependencies.
- **RAII & Drop** to ensure safe removal of the temporary keyfile.
- **Cross‑platform** (Linux, Windows).
- **No legacy scripts** – the entire process is implemented in Rust.

## 📦 Installation
```bash
# Clone the repository
git clone <repo-url> kplock && cd kplock

# Build (release)
cargo build --release
```
The binary will be located at `target/release/kplock`.

## 🛠️ Usage
### Encrypt a keyfile
```bash
./target/release/kplock encrypt -i ./keyfile.key -o ./keyfile.key.age
```
- `-i` : path to the plain‑text keyfile (default `./keyfile.key`).
- `-o` : optional output path for the encrypted file.

### Unlock KeePassXC
```bash
./target/release/kplock unlock
```
The program:
1. Decrypts the encrypted keyfile (configured in `~/.config/kplock/config.toml`).
2. Launches KeePassXC pointing to the temporary keyfile.
3. Waits for KeePassXC to close.
4. Securely wipes the temporary keyfile with overwriting.

## ⚙️ Configuration
On first run it creates `~/.config/kplock/config.toml` with the path to the encrypted keyfile and other parameters. Edit it if you need to change the location.

## 📜 License
This project is licensed under the MIT license.
