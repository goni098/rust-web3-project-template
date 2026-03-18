use std::{fmt::Display, str::FromStr};

use alloy::primitives::{Address, TxHash};
use serde::{Deserialize, Serialize};
use solana_sdk::{pubkey::Pubkey, signature::Signature};

use crate::result::AppErr;

pub mod arg;
pub mod env;
pub mod result;
pub mod tracing;
pub mod util;
pub mod validators;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum UnionAddress {
    Evm(Address),
    Sol(Pubkey),
}

#[derive(Clone, Copy)]
pub enum UnionTxHash {
    Evm(TxHash),
    Sol(Signature),
}

impl FromStr for UnionAddress {
    type Err = AppErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 42 {
            Ok(Self::Evm(s.parse()?))
        } else {
            Ok(Self::Sol(s.parse()?))
        }
    }
}

impl From<Pubkey> for UnionAddress {
    fn from(value: Pubkey) -> Self {
        Self::Sol(value)
    }
}

impl From<Address> for UnionAddress {
    fn from(value: Address) -> Self {
        Self::Evm(value)
    }
}

impl Display for UnionAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Evm(address) => address.fmt(f),
            Self::Sol(address) => address.fmt(f),
        }
    }
}

impl From<TxHash> for UnionTxHash {
    fn from(value: TxHash) -> Self {
        Self::Evm(value)
    }
}

impl From<Signature> for UnionTxHash {
    fn from(value: Signature) -> Self {
        Self::Sol(value)
    }
}

impl Display for UnionTxHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Evm(tx_hash) => tx_hash.fmt(f),
            Self::Sol(signature) => signature.fmt(f),
        }
    }
}
