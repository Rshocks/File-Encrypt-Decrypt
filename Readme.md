# File Encryption with Random Cipher
This project provides a simple way to encrypt a file using one of two encryption algorithms: AES-GCM or ChaCha20-Poly1305. The encryption algorithm is selected randomly each time the program runs.

### Features
- Encrypts files using AES-GCM or ChaCha20-Poly1305.
- Saves the encrypted data, encryption key, and nonce to separate files.
- Supports reading from a plaintext file (`secret.txt` by default) and saving the results to the working directory.

### Prerequisites
- Rust installed on your machine (follow the instructions at https://www.rust-lang.org/tools/install).

### How to Use

- Create a secret.txt file in the root of the project
- Run the program with:
     ```bash
     cargo run
     ```