use alloy::{
    primitives::{Address, address},
    sol,
};

sol!(
    #[allow(missing_docs)]
    #[allow(clippy::too_many_arguments)]
    #[sol(rpc)]
    #[derive(Debug)]
    UniswapPoolV2,
    "src/abi/uniswap_pool_v2.json"
);

pub const WETH_USDC_V2_POOL: Address = address!("0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc");
