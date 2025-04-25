mod encryption;
mod file_handler;
mod decryption;

use rand::Rng;
use encryption::{Encryptor, AesGcmEncryptor, ChaCha20Encryptor};
use decryption::{Decryptor, AesGcmDecryptor, ChaCha20Decryptor};
use file_handler::FileHandler;
use std::io::{self};

fn main() {
    println!("Select an option:");
    println!("1: Encrypt a file");
    println!("2: Decrypt a file");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().expect("Invalid input");

    match choice {
        1 => {
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
        },
        2 => {
            println!("Cipher Method (AES-GCM or ChaCha20)");
            let mut cipher_input = String::new();
            io::stdin().read_line(&mut cipher_input).expect("Failed to read cipher input");
            let cipher_method = cipher_input.trim();
            
            if cipher_method != "AES-GCM" && cipher_method != "ChaCha20" {
                println!("Cipher method must be AES-GCM or ChaCha20.");
                return;
            }

            let decryptor: Box<dyn Decryptor> = if cipher_method == "AES-GCM" {
                println!("Decryption method: AES-GCM");
                Box::new(AesGcmDecryptor)
            } else {
                println!("Decryption method: ChaCha20");
                Box::new(ChaCha20Decryptor)
            };

            println!("Enter the key (in hex):");
            let mut key_input = String::new();
            io::stdin().read_line(&mut key_input).expect("Failed to read key input");
            let key = hex::decode(key_input.trim()).expect("Invalid key format");

            println!("Enter the nonce (in hex):");
            let mut nonce_input = String::new();
            io::stdin().read_line(&mut nonce_input).expect("Failed to read nonce input");
            let nonce = hex::decode(nonce_input.trim()).expect("Invalid nonce format");

            let encrypted_data = FileHandler::read_file("encrypted_data.bin").expect("Failed to read encrypted file");

            match decryptor.decrypt(&key, &nonce, &encrypted_data) {
                Ok(decrypted_data) => {
                    println!("Decrypted {} bytes", decrypted_data.len());
                    FileHandler::save_to_file("decrypted_data.txt", &decrypted_data).expect("Failed to save decrypted data");
                    println!("Decrypted file saved successfully");
                },
                Err(e) => println!("Decryption failed: {}", e),
            }
        },
        _ => {
            println!("Invalid option selected");
        }
    }
}
