#![allow(clippy::too_many_arguments)]

use alloy::sol;

sol!(
    #[sol(rpc)]
    IZkTLSGateway,
    "../target/contracts/IZkTLSGateway.sol/IZkTLSGateway.json"
);
