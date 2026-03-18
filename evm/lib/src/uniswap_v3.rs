use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    UniswapPoolV3,
    "abis/uniswap_pool_v3.json"
);
