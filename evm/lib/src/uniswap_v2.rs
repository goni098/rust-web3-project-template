use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[allow(clippy::too_many_arguments)]
    #[sol(rpc)]
    #[derive(Debug)]
    UniswapPoolV2,
    "abis/uniswap_pool_v2.json"
);
