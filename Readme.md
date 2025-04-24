# File Encryption with Random Cipher
A simple Rust program for encrypting and decrypting files using either AES-GCM or ChaCha20-Poly1305, selected at random during encryption.

### Features
- Encrypts files with a randomly chosen cipher.
- Decrypts using user-provided key and nonce.
- Outputs encrypted data, key, nonce, and cipher method.
- File I/O handled cleanly and modularly via FileHandler.
- CLI prompts for user interaction (mode, key, nonce, etc.).

### Prerequisites
- Rust installed on your machine (follow the instructions at https://www.rust-lang.org/tools/install).

### How to Use

- Create a secret.txt file in the root of the project.
- Run the program with:
     ```bash
     cargo run
     ```
- 1 to encrypt, 2 to decrypt.
- On decryption, provide the correct hex key, hex nonce, and ensure the cipher method matches.