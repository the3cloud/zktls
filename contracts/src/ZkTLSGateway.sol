// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./IZkTLSGateway.sol";
import "zktls-contracts/lib/RequestData.sol";

contract ZkTLSGateway is IZkTLSGateway {
    bytes32 public constant PROVER_ID = keccak256("ExampleProver");

    function buildRequestData() public pure returns (RequestData.RequestDataFull memory requestData) {
        uint64[] memory fields = new uint64[](2);
        fields[0] = 25;
        fields[1] = 39;

        bytes[] memory values = new bytes[](2);
        values[0] = "httpbin.org";
        values[1] = "Close";

        requestData = RequestData.RequestDataFull({
            encryptedOffset: 2,
            fields: fields,
            values: values,
            remote: "httpbin.org:443",
            serverName: "httpbin.org",
            /// This template will store the request data:
            /// "GET /get HTTP/1.1\r\nHost: \r\nConnection: \r\n\r\n"
            requestTemplateHash: 0
        });
    }

    function requestTLSCallTemplate() external returns (bytes32 requestId) {
        RequestData.RequestDataFull memory requestData = buildRequestData();

        bytes memory encodedRequestData = RequestData.encodeRequestDataFull(requestData);

        emit RequestTLSCallBegin(
            requestId,
            PROVER_ID,
            encodedRequestData,
            bytes(""),
            bytes(""),
            1000
        );
    }

    function deliveryResponse(
        bytes32 requestId,
        bytes32 requestHash,
        bytes calldata responseData,
        bytes calldata proof
    ) public {}
}
