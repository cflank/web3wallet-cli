use crate::config;
use crate::errors::{ValidationError, CryptographicError, WalletResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize) ]
pub struct Keystore{
    pub version: String,
    pub metadata: KeystoreMetadata,
    pub crypto: CryptoParams
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystoreMetadata{
    pub alias: Option<String>,
    pub address: String,
    pub created_at: String,
    pub network: String,
    pub keystore_type: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoParams{
    pub cipher: String, //"aes-256-gcm"
    pub ciphertext: String,
    pub cipherparams: CipherParams,
    pub kdf: String,
    pub kdfparams: KdfParams,
    pub mac: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CipherParams {
    /// Initialization vector (hex encoded)
    pub iv: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KdfParams {
    /// Argon2id parameters (preferred)
    Argon2 {        
        dklen: u32,
        memory: u32,
        time: u32,
        parallelism: u32,
        salt: String,
    },
    /// PBKDF2 parameters (legacy compatibility)
    Pbkdf2 {
        dklen: u32,
        c: u32,
        prf: String,
        salt: String,
    },
}

impl Keystore{
    pub fn new(
        alias: Option<String>,
        address: String,
        network: String,
        encrypted_data: Vec<u8>,
        _salt: Vec<u8>,
        nonce: Vec<u8>,
        mac: Vec<u8>,
        kdf_params: KdfParams,
    ) -> Self{
        let metadata = KeystoreMetadata{
            alias,
            address, 
            created_at: chrono::Utc::now().to_rfc3339(),
            network, 
            keystore_type: "web3wallet-cli".to_string(),
        };

        let crypto = CryptoParams{
            cipher: "aes-256-gcm".to_string(),
            ciphertext: hex::encode(encrypted_data),
            cipherparams: CipherParams{
                iv: hex::encode(nonce),
            },
            kdf: match kdf_params{
                KdfParams::Argon2{..} => "argon2id".to_string(),
                KdfParams::Pbkdf2{..} => "pbkdf2".to_string()
            },
            kdfparams: kdf_params,
            mac: hex::encode(mac)
        };

        Self{
            version: "1.0.0".to_string(),
            metadata,
            crypto
        }
    }

    pub fn with_pbkdf2(
        alias: Option<String>,
        address: String,
        network: String,
        encrypted_data: Vec<u8>,
        salt: Vec<u8>,
        nonce: Vec<u8>,
        mac: Vec<u8>,
        iterations: u32,
    ) -> Self {
        let kdf_params = KdfParams::Pbkdf2{
            dklen: config::crypto::KEY_LENGTH as u32,
            c: iterations,
            prf: "hmac-sha256".to_string(),
            salt: hex::encode(&salt)
        };
        Self::new(
            alias,
            address,
            network,
            encrypted_data,
            salt,
            nonce,
            mac,
            kdf_params,
        )
    }
    pub fn with_argon2(
        alias: Option<String>,
        address: String,
        network: String,
        encrypted_data: Vec<u8>,
        salt: Vec<u8>,
        nonce: Vec<u8>,
        mac: Vec<u8>,
        memory: u32,
        iterations: u32,
        parallelism: u32,
    ) -> Self {
        let kdf_params = KdfParams::Argon2 {
            dklen: config::crypto::KEY_LENGTH as u32,
            memory,
            time: iterations,
            parallelism,
            salt: hex::encode(&salt),
        };

        Self::new(
            alias,
            address,
            network,
            encrypted_data,
            salt,
            nonce,
            mac,
            kdf_params,
        )
    }

    /// Get encrypted data as bytes
    pub fn encrypted_data(&self) -> WalletResult<Vec<u8>> {
        hex::decode(&self.crypto.ciphertext).map_err(|e| {
            CryptographicError::DataCorruption {
                details: format!("Invalid ciphertext hex: {}", e),
            }
            .into()
        })
    }

    /// Get salt as bytes
    pub fn salt(&self) -> WalletResult<Vec<u8>> {
        let salt_hex = match &self.crypto.kdfparams {
            KdfParams::Argon2 { salt, .. } => salt,
            KdfParams::Pbkdf2 { salt, .. } => salt,
        };

        hex::decode(salt_hex).map_err(|e| {
            CryptographicError::DataCorruption {
                details: format!("Invalid salt hex: {}", e),
            }
            .into()
        })
    }

    /// Get nonce/IV as bytes
    pub fn nonce(&self) -> WalletResult<Vec<u8>> {
        hex::decode(&self.crypto.cipherparams.iv).map_err(|e| {
            CryptographicError::DataCorruption {
                details: format!("Invalid nonce hex: {}", e),
            }
            .into()
        })
    }

    /// Get MAC as bytes
    pub fn mac(&self) -> WalletResult<Vec<u8>> {
        hex::decode(&self.crypto.mac).map_err(|e| {
            CryptographicError::DataCorruption {
                details: format!("Invalid MAC hex: {}", e),
            }
            .into()
        })
    }

    /// Get KDF parameters
    pub fn kdf_params(&self) -> &KdfParams {
        &self.crypto.kdfparams
    }

    pub fn validate(&self)->WalletResult<()>{
        todo!();
    }

    pub fn to_json(&self)->WalletResult<String>{
        serde_json::to_string_pretty(&self).map_err(|e|{
            ValidationError::InvalidKeystoreSchema {
                error: format!("Json serialization failed: {}", e),
                file_path: "unknow".to_string()
            }
            .into()
        })
    }

    pub fn from_json(json: &str) -> WalletResult<Self>{
        let keystore: Self = serde_json::from_str(json).map_err(|e|{
            ValidationError::InvalidKeystoreSchema{
                error: format!("Json serialization failed: {}", e),
                file_path: "unknow".to_string()
            }
        })?;

        keystore.validate()?;
        Ok(keystore)
    }
}
