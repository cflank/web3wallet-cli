use crate::config;
use crate::errors::{WalletResult, CryptographicError};
use ethers::prelude::*;
use ethers::signers::coins_bip39::{English, Mnemonic};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use zeroize::{Zeroize, ZeroizeOnDrop};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct Wallet {
    #[zeroize(skip)]
    mnemonic: String,
    #[serde(skip)] // 私钥绝不能被序列化或打印  
    master_private_key: Option<Vec<u8>>,
    #[zeroize(skip)]
    address: String,
    #[zeroize(skip)]
    derivation_path: String,
    #[zeroize(skip)]
    network: String,
    #[zeroize(skip)]
    alias: Option<String>,
    #[zeroize(skip)]
    created_at: chrono::DateTime<chrono::Utc>,
}

impl Wallet {
    pub fn from_mnemonic(
        mnemonic: &str,
        network: &str,
        alias: Option<String>
    ) -> WalletResult<Self>{
        let bip_mnemonic = bip39::Mnemonic::from_str(mnemonic).map_err(|e|{
            CryptographicError::InvalidMnemonic{
                detail: e.to_string(),
                suggestion: "Ensure the mnemonic is valid and follows BIP39 standards".to_string()
            }
        })?;

        let wallet = MnemonicBuilder::<English>::default()
            .phrase(mnemonic)
            .build()
            .map_err(|e|{
                CryptographicError::AddressGenerationFailed {
                    details: e.to_string(),
                }
            })?;
        
        Ok(Self{
            mnemonic: mnemonic.to_string(),
            master_private_key: Some(wallet.signer().to_bytes().to_vec()),
            address: format!("{:?}", wallet.address()),
            derivation_path: config::DEFAULT_DERIVATION_PATH.to_string(),
            network: network.to_string(),
            alias,
            created_at: chrono::Utc::now(),
        })    
    }

    pub fn from_private_key(
        private_key: &str,
        network: &str,
        alias: Option<String>
    ) -> WalletResult<Self> {
        crate::utils::validate_private_key(private_key)?;

        let key_str = private_key.strip_prefix("0x").unwrap_or(private_key);
        // Validate private key format
        if key_str.len() != 64 {
            return Err(CryptographicError::InvalidPrivateKey {
                detail: format!("Expected 64 hex characters, got {}", key_str.len()),
                expected: "64 hex characters (with or without 0x prefix)".to_string(),
            }
            .into());
        }

        

        let wallet = key_str.parse::<LocalWallet>().map_err(|e| {
            CryptographicError::InvalidPrivateKey {
                detail: e.to_string(),
                expected: "valid secp256k1 private key".to_string(),
            }
        })?;

        Ok(Self{
            mnemonic: "".to_string(),
            master_private_key: Some(wallet.signer().to_bytes().to_vec()),
            address: format!("{:?}", wallet.address()),
            derivation_path: config::DEFAULT_DERIVATION_PATH.to_string(),
            network: network.to_string(),
            alias,
            created_at: chrono::Utc::now(),
        })
    }

    pub fn has_mnemonic(&self) -> bool {
        !self.mnemonic.is_empty()
    }
    
    pub fn derive_address(&self, index: u32)->WalletResult<DerivedAddress>{
        if self.mnemonic.is_empty() {
            return Err(CryptographicError::KdfFailed {
                details: "Cannot derive addresses from private key only wallet".to_string(),
            }
            .into());
        }

        let derivation_path = format!("{}/{}", self.derivation_path, index);

        let wallet = MnemonicBuilder::<English>::default()
                    .phrase(self.mnemonic.as_str())
                    .derivation_path(&derivation_path)
                    .map_err(|_e| CryptographicError::InvalidDerivationPath {
                        path: derivation_path.clone(),
                        expected: "valid BIP44 derivation path".to_string(),
                    })?
                    .build()
                    .map_err(|e|{
                        CryptographicError::AddressGenerationFailed {
                            details: e.to_string(),
                        }
                    })?;

        Ok(DerivedAddress{
            address: format!("{:?}", wallet.address()),
            index,
            derivation_path,
        })  
    }

    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }
    pub fn address(&self) -> &str {
        &self.address
    }
    pub fn network(&self) -> &str {
        &self.network
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedAddress{
    address: String,
    index: u32,
    derivation_path: String,
}