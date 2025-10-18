use crate::config;
use crate::errors::{WalletResult, UserInputError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OutputFormat {
    Table,
    Json,
}



#[derive (Debug, Clone)]
pub struct BaseComand{
    pub output: OutputFormat,
    pub verbose: bool,
    pub config: Option<PathBuf>,
}


impl Default for BaseComand{
    fn default() -> Self {
        Self{
            output: OutputFormat::Table,
            verbose: false,
            config: None
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateCommand{
    pub base : BaseComand,
    pub words: u8,
    pub save : Option<String>,
    pub network: String,
}


impl CreateCommand{
    pub fn new() -> Self{
        Self {
            base: BaseComand::default(),
            words: config::bip39::DEFAULT_WORD_COUNT,
            save: None,
            network: config::DEFAULT_NETWORK.to_string()
        }
    }

    pub fn validate(&self) -> WalletResult<()>{
        if !config::is_supported_word_count(self.words){
            return Err(UserInputError::InvalidParameters{
                parameter: "words".to_string(),
                value: self.words.to_string(),
                expected: "12 or 24".to_string(), 
            }.into());
        }

        if !config::is_supported_network(&self.network){
            return Err(UserInputError::InvalidNetwork{
                network: self.network.clone(),
                supported: config::SUPPORTED_NETWORKS.iter().map(|s| s.to_string()).collect(),
            }.into());
        }
        if let Some(ref save_path) = self.save {
            crate::utils::validate_file_path(save_path)?;
        }
        Ok(())
    }
}