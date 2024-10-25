use alloy::sol;

sol! {
    #[sol(rpc)]
    contract ZkTLSGateway {
        event RequestTLSCall(bytes data);
    }
}
