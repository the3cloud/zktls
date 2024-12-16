use alloy::sol;

sol! {
    #[sol(rpc)]
    interface ZkTLSGateway {
        function deliverResponse(
            bytes calldata proof_,
            bytes32 proverId_,
            bytes32 responseId_,
            address client_,
            bytes32 dapp_,
            uint64 maxGasPrice_,
            uint64 gasLimit_,
            bytes calldata responses_
        ) external;
    }
}
