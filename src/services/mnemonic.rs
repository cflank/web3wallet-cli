use crate::config;
use crate::errors::{CryptographicError, WalletResult};
use bip39::{Language, Mnemonic};
use rand::RngCore;
use std::str::FromStr;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct SecureMnemonic {
    phrase: String,
}

impl SecureMnemonic {
    pub fn new(phrase: String) -> Self {
        Self { phrase }
    }

    pub fn phrase(&self) -> &str {
        &self.phrase
    }

    pub fn word_count(&self) -> usize {
        self.phrase.split_whitespace().count()
    }

    pub fn words(&self) -> Vec<&str> {
        self.phrase.split_whitespace().collect()
    }

    pub fn word_at(&self, index: usize) -> Option<&str> {
        self.words().get(index).copied()
    }

    pub fn validate(&self) -> WalletResult<()> {
        MnemonicService::validate(&self.phrase)?;
        Ok(())
    }
}
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct SecureSeed {
    bytes: Vec<u8>,
}

impl SecureSeed {
    /// Create new secure seed
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Get seed bytes 
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Get seed length
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Check if seed is empty
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

pub struct MnemonicService;

impl MnemonicService{
    pub fn generate(word_count: u8) -> WalletResult<SecureMnemonic>{
        if !config::is_supported_word_count(word_count){
            return Err(CryptographicError::InvalidAddressFormat{
                details: format!("Unsupported word count: {}", word_count),
                suggestion: "Use 12 or 24 words".to_string()
            }.into())
        }

        let entropy_bits = config::entropy_bits_for_word_count(word_count)
            .ok_or_else(|| CryptographicError::InvalidMnemonic{
                detail: format!("Cannot determinate entropy for {} words", word_count),
                suggestion: "Use 12 or 24 words!".to_string()
            })?;
        
        let mut entropy = vec![0u8; entropy_bits / 8];
        rand::thread_rng().fill_bytes(&mut entropy);
        
        let mnemonic = Mnemonic::from_entropy(&entropy).map_err(|e|{
            CryptographicError::InvalidMnemonic{
                detail: e.to_string(),
                suggestion: "Ensure system has adequate entropy sources".to_string()
            }
        })?;

        entropy.zeroize();
        Ok(SecureMnemonic::new(mnemonic.to_string()))
    }

    pub fn validate(mnemonic_str: &str) -> WalletResult<SecureMnemonic>{
        let mnemonic = Mnemonic::from_str(mnemonic_str).map_err(|e|{
            CryptographicError::InvalidMnemonic{
                detail: e.to_string(),
                suggestion: "Verify the mnemonic phrase has the correct number of words (12 or 24) and all words are from the BIP39 wordlist.".to_string(),
            }
        })?;

        let word_count = mnemonic_str.split_whitespace().count();
        if !config::is_supported_word_count(word_count as u8){
            return Err(CryptographicError::InvalidMnemonic{
                detail: format!("Unsupported word count: {}", word_count),
                suggestion: "Use 12 or 24 words".to_string()
            }
            .into());
        }

        Ok(SecureMnemonic::new(mnemonic.to_string()))
    }

    pub fn generate_seed(mnemonic: &SecureMnemonic, passphrase: Option<&str>) -> WalletResult<SecureSeed>{
        let bip39_mnemonic = Mnemonic::from_str(mnemonic.phrase()).map_err(|e|{
            CryptographicError::InvalidMnemonic{
                detail: e.to_string(),
                suggestion: "Ensure mnemonic is valid BIP39 format".to_string(),
            }
        })?;
        let passphrase = passphrase.unwrap_or("");
        let seed = bip39_mnemonic.to_seed(passphrase);

        Ok(SecureSeed::new(seed.to_vec()))
    }

    ///模拟检查，在实际项目中需要更精确的 OS 检查
    fn check_entropy_availability(required_bits: usize) -> WalletResult<()> {
         if required_bits > 512 {
            return Err(CryptographicError::InsufficientEntropy {
                available: 256, // Simplified value
                required: required_bits as u32,
                suggestion: "Ensure system has adequate entropy sources.".to_string(),
            }
            .into());
        }

        Ok(())
    }
}

