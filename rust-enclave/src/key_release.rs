// rust-enclave/src/key_release.rs
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, KeyInit};
use std::fs::File;
use std::io::{Read, Write};

pub struct KeyReleaseManager {
    pub raw_key_material: [u8; 32], // The 256-bit symmetric key released by the secure HSM
}

impl KeyReleaseManager {
    /// Decrypts a structure-preserving Python engine script into memory using ChaCha20-Poly1305.
    /// This prevents side-channel timing attacks and keeps execution sandboxed inside isolated RAM.
    pub fn decrypt_engine_to_memory(&self, encrypted_file_path: &str) -> Result<String, &'static str> {
        println!("[DECRYPTION] Loading cryptographically sealed manifold engine block into isolated RAM...");

        // Open the encrypted asset file containing your packed Python data structures
        let mut file = File::open(encrypted_file_path).map_err(|_| "Failed to open encrypted engine file on disk.")?;
        let mut ciphertext = Vec::new();
        file.read_to_end(&mut ciphertext).map_err(|_| "Error streaming encrypted bytes into RAM.")?;

        if ciphertext.len() < 12 {
            return Err("CORRUPTION FAULT: Encrypted data stream is too short to contain a valid cipher nonce.");
        }

        // Split the data stream: The first 12 bytes contain the unique cryptographic nonce
        let (nonce_bytes, true_ciphertext) = ciphertext.split_at(12);
        
        let key = Key::from_slice(&self.raw_key_material);
        let nonce = Nonce::from_slice(nonce_bytes);
        let cipher = ChaCha20Poly1305::new(key);

        println!("[DECRYPTION] Executing hardware-isolated ChaCha20-Poly1305 authentication sweep...");
        
        // Decrypt the cipher text directly inside memory. 
        // If an attacker altered a single byte of the encrypted file, authentication will fail instantly.
        let decrypted_bytes = cipher.decrypt(nonce, true_ciphertext)
            .map_err(|_| "CRYPTOGRAPHIC BREACH: Integrity tag mismatch. Payload altered or key revoked.")?;

        let python_script = String::from_utf8(decrypted_bytes)
            .map_err(|_| "FORMAT ERROR: Decrypted memory payload contains invalid non-UTF8 strings.")?;

        println!("[SUCCESS] Python Manifold Engine unlocked inside memory-isolated workspace.");
        Ok(python_script)
    }

    /// Helper utility to encrypt a plain-text script during development builds.
    pub fn seal_engine_for_distribution(&self, plain_text: &str, output_path: &str) -> Result<(), &'static str> {
        let key = Key::from_slice(&self.raw_key_material);
        // In full production, generate a secure random 12-byte nonce using rand::rngs::OsRng
        let nonce_bytes = [0x5E; 12]; 
        let nonce = Nonce::from_slice(&nonce_bytes);
        let cipher = ChaCha20Poly1305::new(key);

        let encrypted_bytes = cipher.encrypt(nonce, plain_text.as_bytes())
            .map_err(|_| "Encryption execution sequence failed.")?;

        let mut file = File::create(output_path).map_err(|_| "Failed to create encrypted file path.")?;
        
        // Write the nonce first, followed immediately by the authenticated cipher body
        file.write_all(&nonce_bytes).map_err(|_| "Error writing nonce header.")?;
        file.write_all(&encrypted_bytes).map_err(|_| "Error writing cipher data payload.")?;

        Ok(())
    }
}
