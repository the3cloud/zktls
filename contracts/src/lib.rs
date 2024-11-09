#![allow(clippy::too_many_arguments)]

use alloy::sol;

sol!(
    #[sol(rpc)]
    ZkTLSGateway,
    "../target/contracts/ZkTLSGateway.sol/ZkTLSGateway.json"
);
