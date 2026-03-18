use alloy::primitives::{Address, address};
use shared::result::AppErr;
use strum::IntoEnumIterator;

pub mod client;
pub mod uniswap_v2;
pub mod uniswap_v3;

#[derive(strum::EnumIter, Debug, Clone, Copy)]
pub enum SupportedChain {
    Mainet = 1,
    Bsc = 56,
}

impl TryFrom<u64> for SupportedChain {
    type Error = AppErr;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        SupportedChain::iter()
            .find(|chain| chain.to_chain_id() == value)
            .ok_or_else(|| AppErr::custom("chain is not supported"))
    }
}

impl SupportedChain {
    pub fn to_chain_id(self) -> u64 {
        self as u64
    }

    pub fn usdt_weth_pool_v3_address(&self) -> Address {
        match self {
            Self::Mainet => address!("0x4e68ccd3e89f51c3074ca5072bbac773960dfa36"),
            Self::Bsc => address!("0x00767e906a751322e1bc26da8be32751c6b75f53"),
        }
    }

    pub fn usdt_weth_pool_v2_address(&self) -> Address {
        match self {
            Self::Mainet => address!("0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852"),
            Self::Bsc => address!("0x8a1Ed8e124fdFBD534bF48baF732E26db9Cc0Cf4"),
        }
    }
}
