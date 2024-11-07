use alloy::sol;

sol!(
    #[sol(rpc)]
    ZkTLSGateway,
    "../target/contracts/ZkTLSGateway.sol/ZkTLSGateway.json"
);
