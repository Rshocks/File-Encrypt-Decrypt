mod encryption;
mod file_handler;

use rand::Rng;
use encryption::{Encryptor, AesGcmEncryptor, ChaCha20Encryptor};
use file_handler::FileHandler;

fn main() {
    let file_path = "secret.txt";
    let data = FileHandler::read_file(file_path).expect("Failed to read file");

    let mut rng = rand::thread_rng();
    let encryptor: Box<dyn Encryptor> = if rng.gen_bool(0.5) {
        println!("Selected cipher: AES-GCM");
        Box::new(AesGcmEncryptor)
    } else {
        println!("Selected cipher: ChaCha20");
        Box::new(ChaCha20Encryptor)
    };

    let ciphertext = encryptor.encrypt(&data);
    println!("Encrypted {} bytes", ciphertext.len());

    FileHandler::save_to_file("encrypted_data.bin", &ciphertext).expect("Failed to save encrypted data");
    println!("Encrypted file saved successfully");
}
