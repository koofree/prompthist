use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key,
};
use argon2::{Argon2, PasswordHasher, password_hash::{rand_core::RngCore, PasswordHash, PasswordVerifier, SaltString}};
use base64::{Engine as _, engine::general_purpose};
use keyring::Entry;
use crate::models::{PromptHistError, Result};

pub struct CryptoManager {
    cipher: Aes256Gcm,
    key_entry: Entry,
}

impl CryptoManager {
    pub fn new() -> Result<Self> {
        let key_entry = Entry::new("prompthist", "encryption_key")
            .map_err(|e| PromptHistError::Encryption(format!("Failed to create keyring entry: {}", e)))?;

        let key = Self::get_or_create_key(&key_entry)?;
        let cipher = Aes256Gcm::new(&key);

        Ok(Self {
            cipher,
            key_entry,
        })
    }

    fn get_or_create_key(key_entry: &Entry) -> Result<Key<Aes256Gcm>> {
        // Try to get existing key from keyring
        match key_entry.get_password() {
            Ok(key_str) => {
                // Decode the base64 key
                let key_bytes = general_purpose::STANDARD
                    .decode(key_str)
                    .map_err(|e| PromptHistError::Encryption(format!("Failed to decode key: {}", e)))?;

                if key_bytes.len() != 32 {
                    return Err(PromptHistError::Encryption("Invalid key length".to_string()));
                }

                Ok(*Key::<Aes256Gcm>::from_slice(&key_bytes))
            }
            Err(_) => {
                // Generate new key
                let mut key_bytes = [0u8; 32];
                OsRng.fill_bytes(&mut key_bytes);

                let key_str = general_purpose::STANDARD.encode(&key_bytes);
                key_entry.set_password(&key_str)
                    .map_err(|e| PromptHistError::Encryption(format!("Failed to store key: {}", e)))?;

                Ok(*Key::<Aes256Gcm>::from_slice(&key_bytes))
            }
        }
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String> {
        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt the data
        let ciphertext = self.cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| PromptHistError::Encryption(format!("Encryption failed: {}", e)))?;

        // Combine nonce and ciphertext
        let mut encrypted_data = Vec::new();
        encrypted_data.extend_from_slice(&nonce_bytes);
        encrypted_data.extend_from_slice(&ciphertext);

        // Encode as base64
        Ok(general_purpose::STANDARD.encode(&encrypted_data))
    }

    pub fn decrypt(&self, encrypted_data: &str) -> Result<String> {
        // Decode from base64
        let encrypted_bytes = general_purpose::STANDARD
            .decode(encrypted_data)
            .map_err(|e| PromptHistError::Encryption(format!("Failed to decode encrypted data: {}", e)))?;

        if encrypted_bytes.len() < 12 {
            return Err(PromptHistError::Encryption("Invalid encrypted data length".to_string()));
        }

        // Extract nonce and ciphertext
        let (nonce_bytes, ciphertext) = encrypted_bytes.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Decrypt the data
        let plaintext = self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| PromptHistError::Encryption(format!("Decryption failed: {}", e)))?;

        String::from_utf8(plaintext)
            .map_err(|e| PromptHistError::Encryption(format!("Invalid UTF-8 in decrypted data: {}", e)))
    }

    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| PromptHistError::Encryption(format!("Password hashing failed: {}", e)))?;

        Ok(password_hash.to_string())
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| PromptHistError::Encryption(format!("Invalid password hash: {}", e)))?;

        let argon2 = Argon2::default();

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn generate_secure_token(&self) -> String {
        let mut token_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut token_bytes);
        general_purpose::STANDARD.encode(&token_bytes)
    }

    pub fn secure_delete_key(&self) -> Result<()> {
        self.key_entry.delete_password()
            .map_err(|e| PromptHistError::Encryption(format!("Failed to delete key: {}", e)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let crypto = CryptoManager::new().unwrap();
        let plaintext = "This is a test message for encryption";

        let encrypted = crypto.encrypt(plaintext).unwrap();
        let decrypted = crypto.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_password_hashing() {
        let crypto = CryptoManager::new().unwrap();
        let password = "test_password_123";

        let hash = crypto.hash_password(password).unwrap();

        assert!(crypto.verify_password(password, &hash).unwrap());
        assert!(!crypto.verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_secure_token_generation() {
        let crypto = CryptoManager::new().unwrap();

        let token1 = crypto.generate_secure_token();
        let token2 = crypto.generate_secure_token();

        assert_ne!(token1, token2);
        assert_eq!(token1.len(), 44); // Base64 encoded 32 bytes
    }
}
