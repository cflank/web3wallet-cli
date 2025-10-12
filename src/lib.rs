


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

// pub type LocalWallet = Wallet<ethers_core::k256::>
