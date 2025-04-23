use aead::{AeadCore, KeyInit};
use aes_gcm::{Aes256Gcm, aead::{Aead, OsRng}};
use chacha20poly1305::ChaCha20Poly1305;

pub trait Encryptor {
    fn encrypt(&self, data: &[u8]) -> Vec<u8>;
}

pub struct AesGcmEncryptor;
pub struct ChaCha20Encryptor;

impl Encryptor for AesGcmEncryptor {
    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let key = Aes256Gcm::generate_key(&mut OsRng);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let cipher = Aes256Gcm::new(&key);
        cipher.encrypt(&nonce, data).expect("encryption failed")
    }
}

impl Encryptor for ChaCha20Encryptor {
    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let cipher = ChaCha20Poly1305::new(&key);
        cipher.encrypt(&nonce, data).expect("encryption failed")
    }
}
