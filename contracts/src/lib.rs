use alloy::sol;

sol! {
    #[sol(rpc)]
    interface IZkTLSDAppCallback {
        function deliveryResponse(bytes32 requestId_, bytes calldata response_) external;
    }
}
