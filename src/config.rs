use std::path::PathBuf;

use crate::config::crypto::{DEFAULT_ARGON2_ITERATIONS, DEFAULT_ARGON2_MEMORY, DEFAULT_ARGON2_PARALLELISM, LOW_MEMORY_ARGON2_ITERATIONS, LOW_MEMORY_ARGON2_MEMORY};

//BIP 44
pub const DEFAULT_DERIVATION_PATH : &str = "m/44'/60'/0'/0";

pub const DEFAULT_NETWORK : &str = "mainnet";

///supported networks
pub const SUPPORTED_NETWORKS: &[&str] = &[
    "mainnet",
    "sepolia",
    "goerli",
    "holesky"
];

pub const DEFAULT_WALLET_DIR : &str = ".web3wallet";

pub const KEYSTORE_EXTENSION: &str = "json";

//Cryptographic configuration
pub mod crypto{
    pub const DEFAULT_ARGON2_MEMORY : u32 = 47_104;
    pub const DEFAULT_ARGON2_ITERATIONS : u32 = 1;
    pub const DEFAULT_ARGON2_PARALLELISM : u32 = 1;

    pub const LOW_MEMORY_ARGON2_MEMORY : u32 = 19_456;
    pub const LOW_MEMORY_ARGON2_ITERATIONS : u32 = 2;

    pub const SALT_LENGTH : usize = 32;

    pub const NONCE_LENGTH : usize = 32;

    pub const KEY_LENGTH : usize = 32;

    pub const MIN_PASSWORD_LENGTH : usize = 8;
}

pub mod fs {
    pub const KEYSTORE_FILE_PERMISSIONS: u32 = 0o600;

}

//BIP 39 configuration
pub mod bip39 {
    pub const SUPPORTED_WORD_COUNTS : &[u8] = &[12, 24];
    pub const DEFAULT_WORD_COUNT : u8 = 12;

    pub const ENTROPY_BITS_12: u32 = 128;
    pub const ENTROPY_BITS_24: u32 = 256;
}

pub fn entropy_bits_for_word_count(count: u8) -> Option<usize> {
    match count {
        12 => Some(bip39::ENTROPY_BITS_12 as usize),
        24 => Some(bip39::ENTROPY_BITS_24 as usize),
        _ => None,
    }
}

pub fn is_supported_word_count(count: u8) -> bool {
    bip39::SUPPORTED_WORD_COUNTS.contains(&count)
}

pub fn is_supported_network(network: &str) -> bool{
    SUPPORTED_NETWORKS.contains(&network)
}

pub fn get_argon2_config(use_low_memory: bool) -> (u32, u32, u32){
    if use_low_memory{
        (
            LOW_MEMORY_ARGON2_MEMORY,
            LOW_MEMORY_ARGON2_ITERATIONS,
            DEFAULT_ARGON2_PARALLELISM
        )
    }else{
        (
            DEFAULT_ARGON2_MEMORY,
            DEFAULT_ARGON2_ITERATIONS,
            DEFAULT_ARGON2_PARALLELISM
        )
    }
}