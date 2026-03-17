use alloy::{
    primitives::{Address, address},
    sol,
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    UniswapPoolV3,
    "src/abi/uniswap_pool_v3.json"
);

pub const WETH_USDT_V3_POOL: Address = address!("0x4e68ccd3e89f51c3074ca5072bbac773960dfa36");
pub const WETH_USDC_V3_POOL: Address = address!("0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8");
