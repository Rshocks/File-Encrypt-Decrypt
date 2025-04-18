use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead, aead::OsRng};
use chacha20poly1305::{aead::AeadCore, ChaCha20Poly1305};
use rand::Rng;
use std::fs::File;
use std::io::{Read, Write, Result};

#[derive(Debug)]
enum CipherType {
    AesGcm,
    ChaCha20,
}

fn read_file(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn random_cipher() -> CipherType {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        CipherType::AesGcm
    } else {
        CipherType::ChaCha20
    }
}

fn encrypt_aes_gcm(data: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let key = Aes256Gcm::generate_key(&mut OsRng);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher = Aes256Gcm::new(&key);
    let ciphertext = cipher.encrypt(&nonce, data).expect("encryption failed");

    (ciphertext, key.to_vec(), nonce.to_vec())
}

fn encrypt_chacha20(data: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let cipher = ChaCha20Poly1305::new(&key);
    let ciphertext = cipher.encrypt(&nonce, data).expect("encryption failed");

    (ciphertext, key.to_vec(), nonce.to_vec())
}

fn save_to_file(path: &str, data: &[u8]) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

fn main() {
    let file_path = "secret.txt";
    let data = read_file(file_path).expect("Failed to read file");

    let cipher = random_cipher();
    println!("Selected cipher: {:?}", cipher);

    let (ciphertext, _key,_noncee) = match cipher {
        CipherType::AesGcm => encrypt_aes_gcm(&data),
        CipherType::ChaCha20 => encrypt_chacha20(&data),
    };
    
    println!("Encrypted {} bytes", ciphertext.len());

    save_to_file("encrypted_data.bin", &ciphertext).expect("Failed to save encrypted data");

    println!("Encrypted file, key, and nonce saved.");
}
