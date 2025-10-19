use crate::errors::{ValidationError, FilesystemError, WalletResult};
use std::path::Path;

pub fn validate_ethereum_address(address: &str) -> WalletResult<()> {

    let addr = address.strip_prefix("0x").unwrap_or(address);

    if addr.len() != 40 {
        return Err(ValidationError::InvalidAddressFormat {
            address: address.to_string(),
            expected: "40 hexadecimal characters".to_string(),
        }.into());
    }

    if !addr.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ValidationError::InvalidAddressFormat {
            address: address.to_string(),
            expected: "hexadecimal characters only".to_string(),
        }
        .into());
    } 
    
    Ok(())
}

pub fn validate_private_key(key: &str) -> WalletResult<()> {
    let private_key = key.strip_prefix("0x").unwrap_or(key);

    if private_key.len() != 64 {
        return Err(ValidationError::InvalidAddressFormat {
            address: key.to_string(),
            expected: "64 hexadecimal characters".to_string(),
        }.into());
    }   

    if !private_key.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ValidationError::InvalidAddressFormat {
            address: key.to_string(),
            expected: "hexadecimal only".to_string(),
        }.into());
    }

    Ok(())
}

pub fn validate_derivation_path(path: &str) -> WalletResult<()> {
    if !path.starts_with("m/") {
        return Err(ValidationError::InvalidAddressFormat {
            address: path.to_string(),
            expected: "path string starting with 'm/'".to_string()
        }.into());
    }

    let components: Vec<&str> = path[2..].split('/').collect();
    for component in components{
        if component.is_empty(){
            return Err(ValidationError::InvalidAddressFormat{
                address: path.to_string(),
                expected: "non-empty path components".to_string()
            }.into());
        }

        let num_str = if component.ends_with("'"){
            &component[..component.len() -1]
        }else{
            component
        };

        if num_str.parse::<u32>().is_err(){
            return Err(ValidationError::InvalidAddressFormat{
                address: path.to_string(),
                expected: "numberic path components".to_string()
            }.into());
        }
    }    
    Ok(())
}

pub fn validate_file_path<P: AsRef<Path>>(path: P) -> WalletResult<()> {
    let path = path.as_ref();

    for component in path.components() {
        if let std::path::Component::ParentDir = component{
            return Err(FilesystemError::PathTraversal{
                path: path.display().to_string(),
            }.into());
        }
    }

    Ok(())
}

