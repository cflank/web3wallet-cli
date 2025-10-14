


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod config;
pub mod errors;
pub mod models;
pub mod services;
pub mod utils;

pub use errors::{WalletError, WalletResult};
pub use models::{Address, Keystore, Wallet};
pub use services::WalletManager;

// pub type LocalWallet = Wallet<ethers_core::k256::>
#[derive(Clone)]
pub struct WalletConfig{
    pub network: String,
    pub wallets_path: std::path::PathBuf,
    pub kdf_iterations: u32,
    pub kdf_memory: u32,
    pub kdf_parallelism: u32,
}

impl Default for WalletConfig{
    fn default() -> Self {
        Self{
            network: "mainnet".to_string(),
            wallets_path: dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from(".")).join(".web3wallet").join("wallets"),
            kdf_iterations: 1,
            kdf_memory: 47_104,
            kdf_parallelism: 1,
        }
    }
}   
