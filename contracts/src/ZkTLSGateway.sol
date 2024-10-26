// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract ZkTLSGateway {
    event RequestTLSCallBegin(bytes32 indexed prover, string url, bytes encrypted_key);

    event RequestTLSCallSegment(bytes data, bool is_encrypted);

    function requestTLSCall(string calldata url, bytes calldata encrypted_key, bytes[] calldata data) public {
        emit RequestTLSCallBegin(0x0, url, encrypted_key);

        for (uint256 i = 0; i < data.length; i++) {
            bool is_encrypted = i % 2 == 0;
            emit RequestTLSCallSegment(data[i], is_encrypted);
        }
    }
}
