use alloy::primitives::Address;
use solana_sdk::pubkey::Pubkey;
use validator::ValidationError;

use crate::UnionAddress;

pub fn validate_solana_pubkey(val: &str) -> Result<(), ValidationError> {
    val.parse::<Pubkey>()
        .map(|_| ())
        .map_err(|_| ValidationError::new("invalid_solana_pubkey"))
}

pub fn validate_evm_address(val: &str) -> Result<(), ValidationError> {
    val.parse::<Address>()
        .map(|_| ())
        .map_err(|_| ValidationError::new("invalid_evm_address"))
}

pub fn validate_union_address(val: &str) -> Result<(), ValidationError> {
    val.parse::<UnionAddress>()
        .map(|_| ())
        .map_err(|_| ValidationError::new("invalid_union_address"))
}

pub fn validate_evm_signature(val: &str) -> Result<(), ValidationError> {
    val.parse::<alloy::signers::Signature>()
        .map(|_| ())
        .map_err(|_| ValidationError::new("invalid_evm_signature"))
}

pub fn validate_solana_signature(val: &str) -> Result<(), ValidationError> {
    val.parse::<solana_sdk::signature::Signature>()
        .map(|_| ())
        .map_err(|_| ValidationError::new("invalid_solana_signature"))
}
