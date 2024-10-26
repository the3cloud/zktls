// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract ZkTLSGateway {
    event RequestTLSCallBegin(bytes32 indexed prover, string url);

    event RequestTLSCallSegment(bytes data, bytes encrypted_key);

    function requestTLSCall(string calldata url, bytes[] calldata data) public {
        emit RequestTLSCallBegin(0x0, url);

        for (uint256 i = 0; i < data.length; i++) {
            emit RequestTLSCallSegment(data[i], "");
        }
    }
}
