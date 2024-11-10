#![allow(clippy::too_many_arguments)]

use alloy::sol;

sol!(
    #[sol(rpc)]
    IZkTLSGateway,
    "../target/contracts/IZkTLSGateway.sol/IZkTLSGateway.json"
);

sol!(
    #[sol(rpc)]
    ZkTLSGateway,
    "../target/contracts/ZkTLSGateway.sol/ZkTLSGateway.json"
);
