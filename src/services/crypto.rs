use crate::config;
use crate::errors::{CryptographicError, WalletResult};
use crate::models::{Keystore, Wallet};
use crate::models::keystore::KdfParams;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use argon2::{Algorithm, Argon2, Params, Version};
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use sha2::Sha256;
use std::path::Path;
use zeroize::{Zeroize, ZeroizeOnDrop};

pub struct CryptoService;

impl CryptoService {
    pub fn encrypt_wallet(
        wallet: &Wallet,
        password:&str,
        use_argon2: bool
    ) -> WalletResult<Keystore> {
        let wallet_data = serde_json::to_vec(wallet).map_err(|e|{
            CryptographicError::KdfFailed{
                details: format!("Wallet serialization failed: {}", e),
            }
        })?;

        let mut salt = vec![0u8; config::crypto::SALT_LENGTH];
        let mut nonce_bytes = vec![0u8; config::crypto::NONCE_LENGTH];

        rand::thread_rng().fill_bytes(&mut salt);
        rand::thread_rng().fill_bytes(&mut nonce_bytes);

        let mut key_bytes = vec![0u8; config::crypto::KEY_LENGTH];

        let kdf_params = if use_argon2 {
            let (memory, iterations, parallelism) = config::get_argon2_config(false);

            Self::derive_key_argon2(
                password.as_bytes(),
                &salt,
                memory,
                iterations,
                parallelism,
                &mut key_bytes
            )?;

            KdfParams::Argon2{
                dklen: config::crypto::KEY_LENGTH as u32,
                memory,
                time: iterations,
                parallelism,
                salt: hex::encode(&salt)
            }

        } else {
            const PBKDF2_ITERATIONS: u32 = 100_000;
            pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, PBKDF2_ITERATIONS, &mut key_bytes);
            KdfParams::Pbkdf2 { 
                dklen: config::crypto::KEY_LENGTH as u32,
                c: PBKDF2_ITERATIONS,
                prf: "hmc-sha256".to_string(),
                salt: hex::encode(&salt)
             }
        };

        let cipher = Aes256Gcm::new_from_slice(&key_bytes).map_err(|e| {
            CryptographicError::KdfFailed {
                details: format!("AES cipher creation failed: {}", e),
            }
        })?;

        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher.encrypt(nonce, wallet_data.as_ref()).map_err(|e| {
            CryptographicError::DecryptionFailed {
                context: format!("Encryption failed: {}", e),
            }
        })?;

        // Create keystore
        let mac = Self::compute_mac(&key_bytes, &ciphertext, &nonce_bytes)?;
        
        // Clear sensitive data
        key_bytes.zeroize();

        Ok(Keystore::new(
            wallet.alias().map(|s| s.to_string()),
            wallet.address().to_string(),
            wallet.network().to_string(),
            ciphertext,
            salt,
            nonce_bytes,
            mac,
            kdf_params
        ))
    }

    ///convert the password to a high-crypto, completely random key
    fn derive_key_argon2(
        password: &[u8],
        salt: &[u8],
        memory: u32,
        iterations: u32,
        parallelism: u32,
        output: &mut [u8]
    ) -> WalletResult<()> {
        let params = Params::new(memory, iterations, parallelism, Some(output.len())).map_err(|e|{
            CryptographicError::KdfFailed{
                details: format!("Invalid Argon2 parameters: {}", e),
            }
        })?;

        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        argon2.hash_password_into(password, salt, output).map_err(|e|{
            CryptographicError::KdfFailed{
                details: format!("Argon2 key failed: {}", e),
            }
        })?;

        Ok(())
    }

    fn compute_mac(key: &[u8], ciphertext: &[u8], nonce: &[u8])->WalletResult<Vec<u8>>{
        use hmac::{Hmac, Mac};

        let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(key).map_err(|e|{
            CryptographicError::KdfFailed{
                details: format!("HMAC key setup failed: {}", e),
            }
        })?;

        mac.update(ciphertext);
        mac.update(nonce);

        Ok(mac.finalize().into_bytes().to_vec())
    }
}