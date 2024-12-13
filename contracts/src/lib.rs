use alloy::sol;

sol! {
    #[sol(rpc)]
    interface IZkTLSDAppCallback {
        /// @notice Callback function that receives the response from a ZkTLS request
        /// @dev This function is called by the ZkTLS gatewal when a response is ready
        /// @param requestId_ The unique identifier of the TLS request
        /// @param response_ The verified response data from the TLS request
        function deliveryResponse(bytes32 requestId_, bytes calldata response_) external;
    }
}
