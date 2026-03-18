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

pub trait BTxHash: ToString {}

pub trait BAddress: ToString {}

impl BTxHash for TxHash {}

impl BAddress for Address {}

impl BTxHash for Signature {}

impl BAddress for Pubkey {}

impl Display for UnionAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Evm(address) => address.fmt(f),
            Self::Sol(address) => address.fmt(f),
        }
    }
}

impl BAddress for UnionAddress {}
