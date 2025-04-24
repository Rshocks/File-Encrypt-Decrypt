use aead::KeyInit;
use aes_gcm::{Aes256Gcm, aead::Aead};
use chacha20poly1305::ChaCha20Poly1305;
use anyhow::Result;

pub trait Decryptor {
    fn decrypt(&self, key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>>;
}

pub struct AesGcmDecryptor;
pub struct ChaCha20Decryptor;

impl Decryptor for AesGcmDecryptor {
    fn decrypt(&self, key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| anyhow::anyhow!("Failed to initialize AES-GCM cipher: {}", e))?;
        let decrypted_data = cipher.decrypt(nonce.into(), ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed with AES-GCM: {}", e))?;
        Ok(decrypted_data)
    }
}

impl Decryptor for ChaCha20Decryptor {
    fn decrypt(&self, key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| anyhow::anyhow!("Failed to initialize ChaCha20 cipher: {}", e))?;
        let decrypted_data = cipher.decrypt(nonce.into(), ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed with ChaCha20: {}", e))?;
        Ok(decrypted_data)
    }
}
