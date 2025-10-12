use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum WalletError {
    #[error("Cryptographic error: {0}")]
    Cryptographic(#[from]CryptographicError),

    #[error("Filesystem error: {0}")]
    Filesystem(#[from]FilesystemError),

    #[error("Input validation error: {0}")]
    UserInput(#[from]UserInputError),

    #[error("Authentication error: {0}")]
    Authentication(#[from]AuthenticationError),

    #[error("I/O error: {0}")]
    Io(String),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("JSON error: {0}")]
    Json(String),
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CryptographicError{
    #[error("CRYPTO_001: Insufficient entropy for secure key generation")]
    InsufficientEntropy{
        available: u32,
        required: u32,
        suggestion: String
    },

    #[error("CRYPTO_002: Invalid BIP39 mnemonic phrase")]
    InvalidMnemonic{
        detail: String,
        suggestion: String
    },

    #[error("CRYPTO_003: Invalid private key format")]
    InvalidPrivateKey{
        detail: String,
        expected: String
    },

    #[error("CRYPTO_004: Invalid BIP39 mnemonic phrase")]
    DecryptionFailed{
        context: String
    },

    
    #[error("CRYPTO_006: Invalid HD derivation path")]
    InvalidDerivationPath {
        /// Provided path
        path: String,
        /// Expected format
        expected: String,
    },

    #[error("CRYPTO_008: Key derivation function failed")]
    KdfFailed {
        /// Error details
        details: String,
    },

    #[error("CRYPTO_010: Address generation failed")]
    AddressGenerationFailed {
        /// Error details
        details: String,
    },

    #[error("CRYPTO_011: Invalid address format")]
    InvalidAddressFormat {
        /// Error details
        details: String,
        /// Suggestion for user
        suggestion: String,
    },

    #[error("CRYPTO_012: Data corruption detected")]
    DataCorruption {
        /// Error details
        details: String,
    },
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum FilesystemError {
    #[error("FS_001: Permission denied for file operation")]
    PermissionDenied{
        path: String,
        operation: String
    },

    #[error("FS_002: File not found")]
    FileNotFound{
        path: String,
        director: String
    },

    #[error("FS_007: Path traversal security violoation")]
    PathTraversal{
        path : String,
    },
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum UserInputError {
    /// Invalid command parameters (INPUT_001)
    #[error("INPUT_001: Invalid command parameters")]
    InvalidParameters {
        parameter: String,
        value: String,
        expected: String,
    },
    
    /// Conflicting command options (INPUT_002)
    #[error("INPUT_002: Conflicting command options")]
    ConflictingOptions {
        option1: String,
        option2: String,
        suggestion: String,
    },

   
    /// Missing required parameter (INPUT_003)
    #[error("INPUT_003: Missing required parameter")]
    MissingParameter {
        parameter: String,
        hint: String,
    },
    
    #[error("INPUT_006: Invalid network specification")]
    InvalidNetwork {
        /// Requested network
        network: String,
        /// Supported networks
        supported: Vec<String>,
    },

    /// Password confirmation mismatch (INPUT_007)
    #[error("INPUT_007: Password confirmation mismatch")]
    PasswordMismatch,

}

/// Authentication errors (AUTH_xxx)
#[derive(Error, Debug, Clone, PartialEq)]
pub enum AuthenticationError {
    /// Wrong password for wallet decryption (AUTH_001)
    #[error("AUTH_001: Incorrect password for wallet decryption")]
    WrongPassword {
        wallet_file: String,
        attempts_remaining: u32,
    },

    /// Password too weak (AUTH_002)
    #[error("AUTH_002: Password does not meet minimum requirements")]
    WeakPassword {
        requirements: Vec<String>,
    },
    
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// Address format validation failed
    #[error("VALIDATION_001: Address format validation failed")]
    InvalidAddressFormat {
        /// Provided address
        address: String,
        /// Expected format
        expected: String,
    },

    /// Keystore schema validation failed
    #[error("VALIDATION_002: Keystore schema validation failed")]
    InvalidKeystoreSchema {
        /// Schema error
        error: String,
        /// File path
        file_path: String,
    },

    /// Command syntax validation failed
    #[error("VALIDATION_003: Command syntax validation failed")]
    InvalidCommandSyntax {
        /// Command
        command: String,
        /// Syntax error
        error: String,
    },

    /// Data integrity check failed
    #[error("VALIDATION_004: Data integrity check failed")]
    IntegrityCheckFailed {
        /// Data type
        data_type: String,
        /// Error details
        details: String,
    },

    /// Version compatibility check failed
    #[error("VALIDATION_005: Version compatibility check failed")]
    VersionIncompatible {
        /// Current version
        current: String,
        /// Required version
        required: String,
    },
}


pub type WalletResult<T> = Result<T, WalletError>;