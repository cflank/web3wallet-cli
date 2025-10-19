use ethers::signers::coins_bip39::mnemonic;

use crate::errors::{WalletResult};
use crate::models::{keystore, Address, Wallet};
use crate::services::{crypto::CryptoService, mnemonic::MnemonicService};
use crate::WalletConfig;
use std::path::Path;

pub struct WalletManager {
    config: WalletConfig,
}

impl WalletManager {
    pub fn new(config: WalletConfig) -> Self {
        Self { config }
    }

    pub async fn create_wallet(&self, word_count: u8) -> WalletResult<Wallet> {
        let mnemonic= MnemonicService::generate(word_count)?;
        Wallet::from_mnemonic(mnemonic.phrase(), &self.config.network, None)
    }

    pub async fn create_wallet_with_network(&self, word_count: u8, network: &str) -> WalletResult<Wallet> {
        let mnemonic= MnemonicService::generate(word_count)?;
        Wallet::from_mnemonic(mnemonic.phrase(), network, None)
    }

    pub async fn import_from_mnemoic(&self, mnemonic_str: &str) -> WalletResult<Wallet> {
        let mnemonic = MnemonicService::validate(mnemonic_str)?;
        Wallet::from_mnemonic(mnemonic.phrase(), &self.config.network, None)
    }

    pub async fn import_from_private_key(&self, private_key: &str) -> WalletResult<Wallet> {
        Wallet::from_private_key(private_key, &self.config.network, None)
    }

    pub async fn save_wallet(&self, wallet: &Wallet, path: &Path, password: &str) -> WalletResult<()>{
        CryptoService::validate_password(password)?;
        let keystore = CryptoService::encrypt_wallet(wallet, password, true)?;
        CryptoService::save_keystore(&keystore, path).await
    }

    pub async fn load_wallet(&self, path: &Path, password: &str) -> WalletResult<Wallet>{
        let keystore = CryptoService::load_keystore(path).await?;
        CryptoService::decrypt_wallet(&keystore, password)
    }

    
}