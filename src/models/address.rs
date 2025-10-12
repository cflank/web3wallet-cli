use crate::config;
use crate::errors::{ValidationError, WalletResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Address{
    address: String, 
    index: Option<u32>,
    derivation_path: Option<String>,
    network: String,
    balance: Option<String>, //in wei
    nonce: Option<u64>,
    label: Option<String>
}

impl Address{
    pub fn new(
        address: String,
        network: String, 
        index: Option<u32>,
        derivation_path: Option<String>
    ) -> WalletResult<Self>{
        crate::utils::validate_ethereum_address(&address)?;
        if !config::is_supported_network(&network){
            return Err(ValidationError::InvalidAddressFormat{
                address: network.clone(),
                expected: format!("one of : {:?}", config::SUPPORTED_NETWORKS)
            }.into());
        }

        if let Some(ref path) = derivation_path{
            crate::utils::validate_derivation_path(path)?;
        }

        Ok(Self{
            address: address,
            index,
            derivation_path,
            balance: None,
            nonce: None,
            label: None,
            network
        })
    }

    pub fn from_string(address: &str, network: &str) -> WalletResult<Self> {
        Self::new(address.to_string(), network.to_string(), None, None)
    }

    pub fn derive(
        address: String, 
        network: String,
        index: u32,
        derivation_path: String
    )-> WalletResult<Self>{
        Self::new(
            address,
            network,
            Some(index),
            Some(derivation_path),
        )
    }

    pub fn address(&self) -> &str{
        &self.address
    }

    // Get short address for display (first 6 + last 4 chars)
    pub fn short_address(&self) -> String {
        if self.address.len() >= 42 {
            format!("{}...{}", &self.address[..6], &self.address[38..])
        } else {
            self.address.clone()
        }
    }

    pub fn validate(&self) -> WalletResult<()>{
        crate::utils::validate_ethereum_address(&self.address)?;

        if !config::is_supported_network(&self.network){
            return Err(ValidationError::InvalidAddressFormat{
                address: self.address.clone(),
                expected: format!("one of {:?}", config::SUPPORTED_NETWORKS)
            }.into());
        }
        
        if let Some(ref path) = self.derivation_path{
            crate::utils::validate_derivation_path(path)?;

            if self.index.is_none(){
                return Err(ValidationError::IntegrityCheckFailed{
                    data_type: "address".to_string(),
                    details: "Derivation path provided without index".to_string(),
                }.into());
            }
            
            if let Some(index) = self.index {
                if !path.ends_with(&format!("/{}", index)) {
                    return Err(ValidationError::IntegrityCheckFailed {
                        data_type: "address".to_string(),
                        details: format!(
                            "Index {} doesn't match derivation path {}",
                            index, path
                        ),
                    }
                    .into());
                }
            }
        }

        Ok(())
    }
}