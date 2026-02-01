use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    UniswapPoolV3,
    "src/abi/uniswap_pool_v3.json"
);
